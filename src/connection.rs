use crate::common::MavMessage;
use crate::{read, write, MavHeader};

use std::sync::Mutex;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::io::{self, Read};

use std::str::FromStr;

use serial::SerialPort;

/// A MAVLink connection
pub trait MavConnection {
    /// Receive a mavlink message.
    ///
    /// Blocks until a valid frame is received, ignoring invalid messages.
    fn recv(&self) -> io::Result<(MavHeader,MavMessage)>;

    /// Send a mavlink message
    fn send(&self, header: &MavHeader, data: &MavMessage) -> io::Result<()>;

    /// Send a message with default header
    fn send_default(&self, data: &MavMessage) -> io::Result<()> {
        let header = MavHeader::get_default_header();
        self.send(&header, data)
    }
}

/// Connect to a MAVLink node by address string.
///
/// The address must be in one of the following formats:
///
///  * `tcp:<addr>:<port>`
///  * `udpin:<addr>:<port>`
///  * `udpout:<addr>:<port>`
///  * `serial:<port>:<baudrate>`
///
/// The type of the connection is determined at runtime based on the address type, so the
/// connection is returned as a trait object.
pub fn connect(address: &str) -> io::Result<Box<MavConnection + Sync + Send>> {
    if address.starts_with("tcp:") {
        Ok(Box::new(Tcp::tcp(&address["tcp:".len()..])?))
    } else if address.starts_with("udpin:") {
        Ok(Box::new(Udp::udpin(&address["udpin:".len()..])?))
    } else if address.starts_with("udpout:") {
        Ok(Box::new(Udp::udpout(&address["udpout:".len()..])?))
    } else if address.starts_with("serial:") {
        Ok(Box::new(Serial::open(&address["serial:".len()..])?))
    } else {
        Err(io::Error::new(
            io::ErrorKind::AddrNotAvailable,
            "Prefix must be one of udpin, udpout, tcp or serial",
        ))
    }
}

struct UdpWrite {
    socket: UdpSocket,
    dest: Option<SocketAddr>,
    sequence: u8,
}

struct PacketBuf {
    buf: Vec<u8>,
    start: usize,
    end: usize,
}

impl PacketBuf {
    pub fn new() -> PacketBuf {
        let mut v = Vec::new();
        v.resize(65536, 0);
        PacketBuf {
            buf: v,
            start: 0,
            end: 0,
        }
    }

    pub fn reset(&mut self) -> &mut [u8] {
        self.start = 0;
        self.end = 0;
        &mut self.buf
    }

    pub fn set_len(&mut self, size: usize) {
        self.end = size;
    }

    pub fn slice(&self) -> &[u8] {
        &self.buf[self.start..self.end]
    }

    pub fn len(&self) -> usize {
        self.slice().len()
    }
}

impl Read for PacketBuf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = Read::read(&mut self.slice(), buf)?;
        self.start += n;
        Ok(n)
    }
}

struct UdpRead {
    socket: UdpSocket,
    recv_buf: PacketBuf,
}

/// UDP MAVLink connection
pub struct Udp {
    read: Mutex<UdpRead>,
    write: Mutex<UdpWrite>,
    server: bool,
}

impl Udp {
    fn new(socket: UdpSocket, server: bool, dest: Option<SocketAddr>) -> io::Result<Udp> {
        Ok(Udp {
            server: server,
            read: Mutex::new(UdpRead {
                socket: socket.try_clone()?,
                recv_buf: PacketBuf::new(),
            }),
            write: Mutex::new(UdpWrite {
                socket: socket,
                dest: dest,
                sequence: 0,
            }),
        })
    }

    pub fn udpin<T: ToSocketAddrs>(address: T) -> io::Result<Udp> {
        let addr = address.to_socket_addrs().unwrap().next().unwrap();
        let socket = UdpSocket::bind(&addr)?;
        Udp::new(socket, true, None)
    }

    pub fn udpout<T: ToSocketAddrs>(address: T) -> io::Result<Udp> {
        let addr = address.to_socket_addrs().unwrap().next().unwrap();
        let socket = UdpSocket::bind(&SocketAddr::from_str("0.0.0.0:0").unwrap())?;
        Udp::new(socket, false, Some(addr))
    }
}

impl MavConnection for Udp {
    fn recv(&self) -> io::Result<(MavHeader, MavMessage)> {
        let mut guard = self.read.lock().unwrap();
        let state = &mut *guard;
        loop {
            if state.recv_buf.len() == 0 {
                let (len, src) = state.socket.recv_from(state.recv_buf.reset())?;
                state.recv_buf.set_len(len);

                if self.server {
                    self.write.lock().unwrap().dest = Some(src);
                }
            }

            if let Ok((h, m)) = read(&mut state.recv_buf) {
                return Ok((h,m));
            }
        }
    }

    fn send(&self, header: &MavHeader, data: &MavMessage) -> io::Result<()> {
        let mut guard = self.write.lock().unwrap();
        let state = &mut *guard;

        let header = MavHeader {
            sequence: state.sequence,
            system_id: header.system_id,
            component_id: header.component_id,
        };

        state.sequence = state.sequence.wrapping_add(1);

        if let Some(addr) = state.dest {
            let mut buf = Vec::new();
            write(&mut buf, header, data)?;
            state.socket.send_to(&buf, addr)?;
        }

        Ok(())
    }
}

/// TCP MAVLink connection
pub struct Tcp {
    read: Mutex<TcpStream>,
    write: Mutex<TcpWrite>,
}

struct TcpWrite {
    socket: TcpStream,
    sequence: u8,
}

impl Tcp {
    pub fn tcp<T: ToSocketAddrs>(address: T) -> io::Result<Tcp> {
        let addr = address.to_socket_addrs().unwrap().next().unwrap();
        let socket = TcpStream::connect(&addr)?;
        Ok(Tcp {
            read: Mutex::new(socket.try_clone()?),
            write: Mutex::new(TcpWrite {
                socket: socket,
                sequence: 0,
            }),
        })
    }
}

impl MavConnection for Tcp {
    fn recv(&self) -> io::Result<(MavHeader, MavMessage)> {
        let mut lock = self.read.lock().unwrap();
        read(&mut *lock).map(|(hdr, pkt)| (hdr,pkt))
    }

    fn send(&self, header: &MavHeader, data: &MavMessage) -> io::Result<()> {
        let mut lock = self.write.lock().unwrap();

        let header = MavHeader {
            sequence: lock.sequence,
            system_id: header.system_id,
            component_id: header.component_id,
        };

        lock.sequence = lock.sequence.wrapping_add(1);

        write(&mut lock.socket, header, data)?;

        Ok(())
    }
}

/// Serial MAVLINK connection
pub struct Serial {
    port: Mutex<::serial::SystemPort>,
    sequence: Mutex<u8>,
}

impl Serial {
    pub fn open(settings: &str) -> io::Result<Serial> {
        let settings: Vec<&str> = settings.split(":").collect();
        let port = settings[0];
        let baud = settings[1].parse::<usize>().unwrap();
        let mut port = ::serial::open(port)?;

        let baud = ::serial::core::BaudRate::from_speed(baud);
        let settings = ::serial::core::PortSettings {
            baud_rate: baud,
            char_size: ::serial::Bits8,
            parity: ::serial::ParityNone,
            stop_bits: ::serial::Stop1,
            flow_control: ::serial::FlowNone,
        };

        port.configure(&settings)?;

        Ok(Serial {
            port: Mutex::new(port),
            sequence: Mutex::new(0),
        })
    }
}

impl MavConnection for Serial {
    fn recv(&self) -> io::Result<(MavHeader, MavMessage)> {
        let mut port = self.port.lock().unwrap();

        loop {
            if let Ok((h, m)) = read(&mut *port) {
                return Ok((h,m));
            }
        }
    }

    fn send(&self, header: &MavHeader, data: &MavMessage) -> io::Result<()> {
        let mut port = self.port.lock().unwrap();
        let mut sequence = self.sequence.lock().unwrap();

        let header = MavHeader {
            sequence: *sequence,
            system_id: header.system_id,
            component_id: header.component_id,
        };

        *sequence = sequence.wrapping_add(1);

        write(&mut *port, header, data)?;
        Ok(())
    }
}
