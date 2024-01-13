#[cfg(feature = "qemu")]
pub mod eth;
#[cfg(not(feature = "qemu"))]
pub mod wifi;
