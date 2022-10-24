use lazy_regex::regex;

use crate::miner::MinerError;

pub static AntminerErrors: [MinerError; 9] = [
    // Unsure
    MinerError {
        re: regex!(r".+load chain ([0-9]).+\n.+(EEPROM error|bad_asic_crc)"),
        msg: "Chain {} EEPROM CRC error",
    },
    MinerError {
        re: regex!(r"Data load fail for chain ([0-9])"),
        msg: "Chain {} load EEPROM fail",
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
        re: regex!(r".+Chain ([0-9]) only find ([0-9]+) asic"),
        msg: "Chain {} only find {} asic",
    },
    MinerError {
        re: regex!(r".+i2c: timeout waiting for bus ready"),
        msg: "I2C timeout",
    },
    MinerError {
        re: regex!(r".+fail to read pic temp for chain ([0-9])"),
        msg: "Chain {} read pic temp fail",
    },
];
