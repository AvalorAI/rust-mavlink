# rust-mavlink

## Avalor Fork Modifications

This branch splits from rust-mavlink, continuing from version 0.12.2. It fixes fields that should be bitmasks being recognized as enums. This leads to messages with bitmasks that have multiple bits set being discarded wrongfully.

This is fixed (?) in their 0.13+ version but that one has other issues in the form of dissapearing extension fields that we need. Since this was an easy and quick fix we currently use this fork.

If 0.13 gets updated to once again give access to the extension fields we need, and also support the newest version of the mavlink standard (currently the parser has a bug with ILLUMINATOR_MODE being used as both an enum and bitmask) we should remove this fork and use the main repo again.

Two additional MavCmds namely:

- `AVALOR_CUSTOM_AUTERION_FLAP_CHECK` is added to support flap checks on vehicles which have a version lower than 3.0.0

- `MAV_CMD_EXTERNAL_POSITION_ESTIMATE` is added to be able to correct the vehicle position when GPS is not available.

## Info

[![Build status](https://github.com/mavlink/rust-mavlink/actions/workflows/test.yml/badge.svg)](https://github.com/mavlink/rust-mavlink/actions/workflows/test.yml)
[![Crate info](https://img.shields.io/crates/v/mavlink.svg)](https://crates.io/crates/mavlink)
[![Documentation](https://docs.rs/mavlink/badge.svg)](https://docs.rs/mavlink)

Rust implementation of the [MAVLink](https://mavlink.io/en) UAV messaging protocol,
with bindings for all message sets.

Add to your Cargo.toml:

```
mavlink = "0.10.1"
```

## Examples

See [src/bin/mavlink-dump.rs](src/bin/mavlink-dump.rs) for a usage example.

It's also possible to install the working example via `cargo` command line:

```sh
cargo install mavlink
```

### Community projects

Check some projects built by the community:

- [mavlink2rest](https://github.com/patrickelectric/mavlink2rest): A REST server that provides easy and friendly access to mavlink messages.
- [mavlink-camera-manager](https://github.com/mavlink/mavlink-camera-manager): Extensible cross-platform camera server.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.
