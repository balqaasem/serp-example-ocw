
#![cfg_attr(not(feature = "std"), no_std)]

pub mod ocw;
pub mod price_feed;

use crate::tests;
#[cfg(test)]
mod tests;

pub use ocw::*;
pub use price_feed::FetchPrice;