use crate::parser::MavEnumEntry;

pub fn get_custom_entries() -> Vec<MavEnumEntry> {
    vec![
        MavEnumEntry {
            value: Some(247),
            name: "CUSTOM_AUTERION_FLAP_CHECK".to_string(),
            description: Some("Custom message for flap checks on auterion devices".to_string()),
            params: None,
        },
        MavEnumEntry {
            value: Some(31100),
            name: "STARLINK".to_string(),
            description: Some("Send position to starlink".to_string()),
            params: Some(vec!["latitude".to_string(), "longitude".to_string()]),
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
        MavEnumEntry {
            value: Some(31050),
            name: "CUSTOM_PATRON_CANCEL_LMT".to_string(),
            description: Some("Cancels LMT, tracking stays enabled".to_string()),
            params: None,
        },
        MavEnumEntry {
            value: Some(31051),
            name: "CUSTOM_PATRON_START_TRACKING".to_string(),
            description: None,
            params: None,
        },
        MavEnumEntry {
            value: Some(31052),
            name: "CUSTOM_PATRON_START_LMT".to_string(),
            description: None,
            params: None,
        },
        MavEnumEntry {
            value: Some(31053),
            name: "CUSTOM_PATRON_TRACKER_TYPE".to_string(),
            description: Some("Switches tracker type, 1: ViT, 2: MedianFlow".to_string()),
            params: Some(vec!["type (1/2)".to_string()]),
        },
        MavEnumEntry {
            value: Some(31054),
            name: "CUSTOM_PATRON_SELECTION_MODE".to_string(),
            description: Some("Switches object selection mode, 1: SOT, 2: AI-Assisted".to_string()),
            params: Some(vec!["mode (1/2)".to_string()]),
        },
        MavEnumEntry {
            value: Some(31055),
            name: "CUSTOM_PATRON_AUTO_LMT".to_string(),
            description: Some("Enables AI Auto lmt".to_string()),
            params: Some(vec!["enable/disable".to_string()]),
        },
        MavEnumEntry {
            value: Some(31056),
            name: "CUSTOM_PATRON_VIS_NAV_FOLLOW".to_string(),
            description: Some("Enables follow-lmt sequence".to_string()),
            params: Some(vec!["enable/disable".to_string()]),
        },
        MavEnumEntry {
            value: Some(31057),
            name: "CUSTOM_PATRON_AUTO_REC".to_string(),
            description: Some("Enables auto recording during lmt".to_string()),
            params: Some(vec!["enable/disable".to_string()]),
        },
        MavEnumEntry {
            value: Some(31058),
            name: "CUSTOM_PATRON_DATA_ACQ".to_string(),
            description: Some("Enables dataset acquistion routine".to_string()),
            params: Some(vec!["enable/disable".to_string()]),
        },
    ]
}
