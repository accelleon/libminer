pub mod common;

#[cfg(feature = "avalon")]
pub mod avalon;
#[cfg(feature = "minerva")]
pub mod minerva;
#[cfg(feature = "antminer")]
pub mod antminer;
#[cfg(feature = "whatsminer")]
pub mod whatsminer;
