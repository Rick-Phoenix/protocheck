#![allow(clippy::doc_overindented_list_items)]

include!("./google.rpc.rs");

#[cfg(feature = "serde")]
mod rpc_serde_impls;
