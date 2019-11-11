//! Tendermint is a high-performance blockchain consensus engine that powers
//! Byzantine fault tolerant applications written in any programming language.
//! This crate provides core types for representing information about Tendermint
//! blockchain networks, including chain information types, secret connections,
//! and remote procedure calls (JSONRPC).

#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tendermint/kms/master/img/tendermint.png",
    html_root_url = "https://docs.rs/tendermint/0.10.0"
)]

// NOTE(EB): can't figure out how to easily remove the extern crate per Rust2018 upgrade ...
#[allow(unused_extern_crates)]
extern crate prost_amino as prost;

// NOTE(EB): can't figure out how to easily remove the extern crate per Rust2018 upgrade ...
#[allow(unused_extern_crates)]
#[macro_use]
extern crate prost_amino_derive as prost_derive;

#[macro_use]
pub mod error;

pub mod abci;
pub mod account;
pub mod amino_types;
pub mod block;
pub mod chain;
pub mod channel;
pub mod config;
pub mod consensus;
pub mod evidence;
pub mod genesis;
pub mod hash;
pub mod merkle;
mod moniker;
pub mod net;
pub mod node;
pub mod private_key;
pub mod public_key;
pub mod rpc;
mod serializers;
pub mod signature;
pub mod time;
mod timeout;
pub mod validator;
mod version;
pub mod vote;

pub use crate::genesis::Genesis;
pub use crate::{
    block::Block,
    error::{Error, ErrorKind},
    hash::Hash,
    moniker::Moniker,
    public_key::{PublicKey, TendermintKey},
    signature::Signature,
    time::Time,
    timeout::Timeout,
    version::Version,
    vote::Vote,
};
pub use private_key::PrivateKey;
