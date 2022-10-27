use lazy_regex::regex;

use crate::miner::MinerError;

pub static MineraErrors: [MinerError; 4] = [
    MinerError {
        re: regex!(r"power up to.+failed read_bak"),
        msg: "PSU failed to power up",
    },
    MinerError {
        re: regex!(r"ACK not found"),
        msg: "SPI ACK not found",
    },
    MinerError {
        re: regex!("low fan speed of fan ([0-9])"),
        msg: "Fan {} speed too low",
    },
    MinerError {
        re: regex!("C3012 ([0-9]) failure, disabling!"),
        msg: "Chain {} failure",
    },
];

pub static MinerVaErrors: [MinerError; 7] = [
    MinerError {
        re: regex!(r".+Error: fan ([0-9]) failed"),
        msg: "Fan {} failed",
    },
    MinerError {
        re: regex!(r".+booting board ([0-9]).+\n.+ACK not found"),
        msg: "Board {} ACK not found",
    },
    MinerError {
        re: regex!(r".+(voltage not up to standard|电源故障，电压不达标)"),
        msg: "Voltage not up to standard",
    },
    MinerError {
        re: regex!(r".+Error: init power supply"),
        msg: "Unable to init power supply",
    },
    MinerError {
        re: regex!(r".+(?:init chip|启动芯片|初始化芯片)([0-9])/([0-9])"),
        msg: "Failed to init board {} chip {}",
    },
    MinerError {
        re: regex!(r".+mv64xxx_i2c_fsm: Ctlr Error"),
        msg: "I2C controller error",
    },
    MinerError {
        re: regex!(r".+Stratum connection to pool [0-9] interrupted.+\n.+flushing server.+\n.+flush failed"),
        msg: "Connection interrupted, failed to flush server",
    },
];
