use lazy_regex::regex;

use crate::miner::MinerError;

pub static MinerVaErrors: [MinerError; 5] = [
    MinerError {
        re: regex!(r".+Error: fan ([0-9]) failed"),
        msg: "Fan {} failed",
    },
    MinerError {
        re: regex!(r".+booting board ([0-9]).+\n.+ACK not found"),
        msg: "Board {} ACK not found",
    },
    MinerError {
        re: regex!(r".+voltage not up to standard"),
        msg: "Voltage not up to standard",
    },
    MinerError {
        re: regex!(r".+Error: init power supply"),
        msg: "Unable to init power supply",
    },
    MinerError {
        re: regex!(r".+init chip([0-9])/([0-9])"),
        msg: "Failed to init board {} chip {}",
    }
];