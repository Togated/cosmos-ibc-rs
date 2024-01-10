//! This crate functions as an intermediary layer between the storage of host
//! chains and an IBC client implementation, providing developers with necessary
//! traits to craft their custom light clients. It streamlines the process of
//! integrating light clients with the host, enabling interaction with the store
//! for pertinent client state transitions.
#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types))]
#![deny(
    warnings,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]

#[cfg(feature = "std")]
extern crate std;

pub mod client_state;
pub mod consensus_state;

mod context;
pub use context::*;

/// Re-exports convenient derive macros from `ibc-derive` for implementing the
/// [`ClientState`] and [`ConsensusState`] traits.
pub mod derive {
    /// A derive macro for implementing the [`ClientState`] trait for enums. Enums
    /// with variants that also implement the [`ClientState`] trait can leverage
    /// this macro for automatic implementation.
    ///
    /// To specify the generic arguments for [`ClientState`], use the following
    /// attributes:
    /// - `#[validation(<YourClientValidationContext>)]`
    /// - `#[execution(<YourClientExecutionContext>)]`
    ///
    /// When utilizing the `ibc` meta-crate, apply this macro with `IbcClientState`.
    /// For the `ibc-core` crate, utilize `IbcCoreClientState`.
    pub use ibc_derive::{IbcClientState, IbcCoreClientState};
    /// A derive macro for implementing the [`ConsensusState`] trait for enums.
    /// Enums with variants that also implement the [`ConsensusState`] trait can
    /// leverage this macro for automatic implementation.
    ///
    /// When utilizing the `ibc` meta-crate, apply this macro with
    /// `IbcConsensusState`. For the `ibc-core` crate, utilize
    /// `IbcCoreConsensusState`.
    pub use ibc_derive::{IbcConsensusState, IbcCoreConsensusState};
}

pub mod types {
    #[doc(inline)]
    pub use ibc_core_client_types::*;
}
