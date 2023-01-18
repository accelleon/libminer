use serde::Deserialize;
use lazy_regex::regex;
use crate::{
    error::Error,
    miners::common::{StatusResp, StatusCode}
};

#[derive(Debug, Deserialize)]
#[serde(from = "[u32; 6]")]
pub struct PowerSupplyInfo {
    pub err: u32,
    pub volt_cntrl: f32,
    pub volt_hash: f32,
    pub current: u32,
    pub power: u32,
    pub set_volt_hash: f32,
}

impl<'a> From<[u32; 6]> for PowerSupplyInfo {
    fn from(v: [u32; 6]) -> Self {
        PowerSupplyInfo {
            err: v[0],
            volt_cntrl: v[1] as f32 / 100.0,
            volt_hash: v[2] as f32 / 100.0,
            current: v[3],
            power: v[4],
            set_volt_hash: v[5] as f32 / 100.0,
        }
    }
}

impl<'a> TryFrom<&'a str> for PowerSupplyInfo {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let re = regex!(r"PS\[(\d+) (\d+) (\d+) (\d+) (\d+) (\d+)\]");
        let caps = re.captures(&input).ok_or(Error::InvalidResponse)?;
        let caps = caps.iter().skip(1).map(|c| c.unwrap().as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();
        Ok(Self {
            err: caps[0],
            volt_cntrl: caps[1] as f32 / 100.0,
            volt_hash: caps[2] as f32 / 100.0,
            current: caps[3],
            power: caps[4],
            set_volt_hash: caps[5] as f32 / 100.0,
        })
    }
}

impl TryFrom<StatusResp> for PowerSupplyInfo {
    type Error = Error;

    fn try_from(status: StatusResp) -> Result<Self, Self::Error> {
        let status = &status.status[0];
        if status.status != StatusCode::INFO {
            return Err(Error::ApiCallFailed(status.msg.clone()));
        }

        // Parse message
        // ASC 0 set info: PS[0 1197 1249 260 3247 1248]
        let re = regex!(r"ASC \d+ set info: (PS\[\d+ \d+ \d+ \d+ \d+ \d+\])");
        let caps = re.captures(&status.msg).ok_or(Error::InvalidResponse)?;
        Self::try_from(caps.get(1).unwrap_or_else(|| unreachable!()).as_str())
    }
}

impl PowerSupplyInfo {
    pub fn get_cmd() -> &'static str {
        r#"{"command":"ascset","parameter":"0,hashpower"}"#
    }

    pub fn set_cmd(sleep: bool) -> String {
        format!(r#"{{"command":"ascset","parameter":"0,hashpower,{}"}}"#, if sleep { 0 } else { 1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_good_response() {
        let input = r#"{"STATUS":[{"STATUS":"I","When":10075,"Code":118,"Msg":"ASC 0 set info: PS[0 1197 1249 260 3247 1248]","Description":"cgminer 4.11.1"}],"id":1}"#;
        let status: StatusResp = serde_json::from_str(input).unwrap();
        let hashpower = PowerSupplyInfo::try_from(status).unwrap();
        assert_eq!(hashpower.err, 0);
        assert_eq!(hashpower.volt_cntrl, 11.97);
        assert_eq!(hashpower.volt_hash, 12.49);
        assert_eq!(hashpower.current, 260);
        assert_eq!(hashpower.power, 3247);
        assert_eq!(hashpower.set_volt_hash, 12.48);
    }

    #[test]
    fn it_parses_bad_response() {
        let input = r#"{"STATUS":[{"STATUS":"E","When":10075,"Code":118,"Msg":"ASC 0 set info: PS[0 1197 1249 260 3247 1248]","Description":"cgminer 4.11.1"}],"id":1}"#;
        let status: StatusResp = serde_json::from_str(input).unwrap();
        let hashpower = PowerSupplyInfo::try_from(status);
        assert!(hashpower.is_err());
    }

    #[test]
    fn it_parses() {
        use crate::miners::avalon::cgminer::de;
        let input = "PS[0 1197 1249 260 3247 1248]";

        #[derive(Debug, Deserialize)]
        struct Test {
            pub PS: PowerSupplyInfo,
        }

        let test: Test = de::from_str(input).unwrap();
        assert_eq!(test.PS.err, 0);
        assert_eq!(test.PS.volt_cntrl, 11.97);
        assert_eq!(test.PS.volt_hash, 12.49);
        assert_eq!(test.PS.current, 260);
        assert_eq!(test.PS.power, 3247);
        assert_eq!(test.PS.set_volt_hash, 12.48);
    }
}
