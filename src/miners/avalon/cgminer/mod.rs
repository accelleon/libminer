mod de;
mod asc;
pub use asc::*;
mod estats;
pub use estats::*;
mod version;
pub use version::*;

pub use de::Error;
pub use crate::common::{
    StatsResp, Stats, StatusCode, StatusResp
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_status() {
        let s = r#"{"STATUS":[{"STATUS":"I","When":57242,"Code":118,"Msg":"ASC 0 set info: LED[0]","Description":"cgminer 4.11.1"}],"id":1}"#;
        let _v: StatusResp = from_str(s).unwrap();
    }
}
