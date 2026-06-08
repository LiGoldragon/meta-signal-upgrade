//! Schema-derived meta-signal contract for the `upgrade` policy surface.
//!
//! The public API of this crate is the generated wire contract: typed
//! `Input` / `Output` roots, payload records, route enums, short-header
//! codecs, and `signal-frame` request/reply aliases. Runtime policy
//! storage and selector execution live in `upgrade`.

pub mod schema {
    #[rustfmt::skip]
    pub mod lib;
}

pub use schema::lib::*;
