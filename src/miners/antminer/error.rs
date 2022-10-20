use lazy_regex::regex;

use crate::miner::MinerError;

pub static AntminerErrors: [MinerError; 4] = [
    MinerError {
        re: regex!(r".+load chain ([0-9]).+\n.+EEPROM error"),
        msg: "Chain {} EEPROM error",
    },
    MinerError {
        re: regex!(r".+ERROR_FAN_LOST"),
        msg: "Fan lost",
    },
    MinerError {
        re: regex!(r".+ERROR_TEMP_TOO_HIGH"),
        msg: "Temperature too high",
    },
    MinerError {
        re: regex!(r".+_read_an6_voltage"),
        msg: "Read voltage failed",
    },
];