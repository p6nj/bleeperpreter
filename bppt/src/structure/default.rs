use crate::Notes;
use serde_json::from_str;

impl Default for Notes {
    fn default() -> Self {
        from_str(
            r#"
        {
            "set": "aAbcCdDefFgG",
            "score": "cccd'ed`cedd''c"
        }
        "#,
        )
        .unwrap()
    }
}
