use crate::parser::MavEnumEntry;

pub fn get_custom_entries() -> Vec<MavEnumEntry> {
    vec![
        MavEnumEntry {
            value: Some(218),
            name: "GURZUF_ARM_DISARM".to_string(),
            description: Some("Gurzuf arm/disarm command".to_string()),
            params: Some(vec![
                "action_type".to_string(),
                "enable/disable".to_string(),
            ]),
        },
        MavEnumEntry {
            value: Some(31100),
            name: "GURZUF_STARLINK".to_string(),
            description: Some("Send posistoin to starlink".to_string()),
            params: Some(vec!["lat".to_string(), "long".to_string()]),
        },
        MavEnumEntry {
            value: Some(247),
            name: "CUSTOM_AUTERION_FLAP_CHECK".to_string(),
            description: Some("Custom message for flap checks on auterion devices".to_string()),
            params: None,
        },
        MavEnumEntry {
            value: Some(43003),
            name: "MAV_CMD_EXTERNAL_POSITION_ESTIMATE".to_string(),
            description: Some(
                "Provide an external position estimate for use when dead-reckoning. This is meant \
                 to be used for occasional position resets that may be provided by an external \
                 system such as a remote pilot using landmarks over a video link."
                    .to_string(),
            ),
            params: Some(vec![
                "transmission_time".to_string(),
                "processing_time".to_string(),
                "accuracy".to_string(),
                "param4".to_string(),
                "latitude".to_string(),
                "longitude".to_string(),
                "altitude".to_string(),
            ]),
        },
    ]
}
