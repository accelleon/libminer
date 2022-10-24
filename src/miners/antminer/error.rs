use lazy_regex::regex;

use crate::{miner::MinerError, Miner};

pub static AntminerErrors: [MinerError; 6] = [
    // Unsure
    MinerError {
        re: regex!(r".+load chain ([0-9]).+\n.+EEPROM error"),
        msg: "Chain {} EEPROM error",
    },
    MinerError {
        re: regex!(r".+ERROR_POWER_LOST"),
        msg: "Power lost",
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
    MinerError {
        re: regex!(r".+Chain ([0-9]) only find ([0-9]) asic"),
        msg: "Chain {} only find {} asic",
    },
];
