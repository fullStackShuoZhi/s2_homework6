//! Low-level types used throughout the Substrate stencil code.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};

/// Type used for expressing timestamp.
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;


/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Block ID.
pub type BlockId = generic::BlockId<Block>;