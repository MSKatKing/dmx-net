#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serial")]
mod serial;

#[cfg(feature = "serial")]
pub use serial::*;

#[cfg(test)]
mod tests {

}
