#![allow(clippy::all)]
#![allow(clippy::many_single_char_names)]
#![deny(missing_docs)]
// TODO: when https://git.io/JYTnW gets stabilized add the readme as module docs.

//! Scuttlebutt provides many utility functions for cryptographic applications.

/// This is a legacy alias for `U8x16`
pub type Block = vectoreyes::U8x16;

mod block512;
/// Module for encapsulating communication channels for `swanky`.
pub mod channel;
pub mod cointoss;
mod hash_aes;
pub use swanky_serialization as serialization;
pub mod utils;

/// A polyfill for the `swanky-field*` family of crates.
pub mod field {
    pub use swanky_field::{
        field_ops, Degree, DegreeModulo, FiniteField, IsSubFieldOf, PrimeFiniteField,
    };
    pub use swanky_field_binary::*;
    pub use swanky_field_f61p::*;
    pub use swanky_field_ff_primes::*;
    pub use swanky_field_fft as fft;
}
/// A polyfill for the ring functionality inside of `swanky-field`.
pub mod ring {
    pub use swanky_field::{ring_ops, FiniteRing, IsSubRingOf};
}

pub use crate::{
    block512::Block512,
    channel::{AbstractChannel, Channel, HashChannel, SymChannel, SyncChannel, TrackChannel},
    hash_aes::{AesHash, AES_HASH},
};
pub use swanky_aes_rng::{AesRng, UniformIntegersUnderBound};

#[cfg(unix)]
pub use crate::channel::{
    track_unix_channel_pair, unix_channel_pair, TrackUnixChannel, UnixChannel,
};

/// A marker trait denoting that the given scheme is semi-honest secure.
pub trait SemiHonest {}
/// A marker trait denoting that the given scheme is maliciously secure.
pub trait Malicious: SemiHonest {}
