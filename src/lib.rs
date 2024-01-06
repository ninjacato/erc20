#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

//! A simple implementation for parsing ERC20 transactions

extern crate serde;

/// ERC20 specific information.
pub mod erc20;
#[cfg(test)]
mod erc20_tests;
mod error;
/// web3 transaction specific operations.
pub mod transaction;
#[cfg(test)]
mod transaction_tests;
/// Ethereum transfer abstraction.
pub mod transfer;
/// A set of useful methods and abstractions.
pub mod util;
#[cfg(test)]
mod util_tests;

pub use self::error::ERC20Error;
pub use web3::types::{Index, Transaction, H160, H256, U256, U64};
