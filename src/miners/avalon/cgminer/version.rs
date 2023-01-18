use serde::Deserialize;
use lazy_regex::regex;

use crate::error::Error;
use super::Status;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Version {
    #[serde(rename = "CGMiner")]
    pub cgminer: String,
    pub api: String,
    pub stm8: String,
    pub prod: String,
    pub model: String,
    pub hwtype: String,
    pub swtype: String,
    pub version: String,
    pub loader: String,
    pub dna: String,
    pub mac: String,
    pub upapi: String,
}

impl Version {
    pub fn model(&self) -> Result<&str, Error> {
        let re = regex!(r"([\w\d]+)-(?:\d+)");
        let caps = re.captures(&self.model).ok_or(Error::InvalidResponse)?;
        Ok(caps.get(1).unwrap_or_else(|| unreachable!()).as_str())
    }

    pub fn hashrate_th(&self) -> Result<f64, Error> {
        let re = regex!(r"(?:[\d\w]+)-(\d+)");
        let caps = re.captures(&self.model).ok_or(Error::InvalidResponse)?;
        Ok(caps.get(1).unwrap_or_else(|| unreachable!()).as_str().parse().unwrap_or_else(|_| unreachable!()))
    }

    pub fn mac_addr(&self) -> String {
        // Need to add colons
        self.mac.chars().enumerate().fold(String::new(), |mut acc, (i, c)| {
            if i % 2 == 0 && i != 0 {
                acc.push(':');
            }
            acc.push(c);
            acc
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct VersionResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
    #[serde(rename = "VERSION")]
    pub version: Option<Vec<Version>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let s = r#"{"STATUS":[{"STATUS":"S","When":11849,"Code":22,"Msg":"CGMiner versions","Description":"cgminer 4.11.1"}],"VERSION":[{"CGMiner":"4.11.1","API":"3.7","STM8":"20.08.01","PROD":"AvalonMiner 1246-81","MODEL":"1246-81","HWTYPE":"MM3v2_X3","SWTYPE":"MM314","VERSION":"21030201_4ec6bb0_09b1765","LOADER":"d0d779de.00","DNA":"020100000828a153","MAC":"b4a2eb3460fa","UPAPI":"2"}],"id":1}"#;
        let v: VersionResp = serde_json::from_str(s).unwrap();
        assert_eq!(v.version.unwrap()[0].model().unwrap(), "1246");
    }
}
