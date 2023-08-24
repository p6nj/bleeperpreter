use super::*;

pub(super) fn mixed_root(bpm: u8, lengths: (u8, u8)) -> usize {
    from_str::<Root>(
        format!(
            r#"{{
        "My First Album": {{
            "artist": "me",
            "tracks": {{
                "My First Song": {{
                    "BPM": {},
                    "channels": {{
                        "piano": {{
                            "signal": "4*abs(f*t-floor(f*t+1/2))-1",
                            "set": "aAbcCdDefFgG",
                            "score": "@4${}!100 .",
                            "tuning": 442
                        }},
                        "synth": {{
                            "signal": "sin((2*pi*f*t)-sin(2*pi*8*t))",
                            "set": "aAbcCdDefFgG",
                            "tuning": 442,
                            "score": "@4${}!100 a"
                        }}
                    }}
                }}
            }}
        }}
    }}"#,
            bpm, lengths.0, lengths.1
        )
        .as_str(),
    )
    .unwrap()
    .mix()
    .unwrap()
    .get("My First Album")
    .unwrap()
    .1
    .get("My First Song")
    .unwrap()
    .len()
}

pub(super) fn custom_mask(mask: &str) -> usize {
    from_str::<Root>(
        format!(
            r#"{{
        "My First Album": {{
            "artist": "me",
            "tracks": {{
                "My First Song": {{
                    "BPM": 60,
                    "channels": {{
                        "piano": {{
                            "signal": "4*abs(f*t-floor(f*t+1/2))-1",
                            "set": "aAbcCdDefFgG",
                            "score": "{}",
                            "tuning": 442
                        }}
                    }}
                }}
            }}
        }}
    }}"#,
            mask
        )
        .as_str(),
    )
    .unwrap()
    .mix()
    .unwrap()
    .get("My First Album")
    .unwrap()
    .1
    .get("My First Song")
    .unwrap()
    .len()
}
