//! Official API doc: [pinning-services-api-spec](https://ipfs.github.io/pinning-services-api-spec).
//!
//! ## Error Handling
//!
//!
//! ## About `requestid`
//! [Here the original](https://ipfs.github.io/pinning-services-api-spec/#section/Provider-hints/Optimizing-for-speed-and-connectivity). \
//! `requestid` is an unique identifier of a pin request. \
//! When a pin is created, the service responds with unique requestid that can be later used for pin removal.
//! When the same cid is pinned again,
//! a different requestid is returned to differentiate between those pin requests. \
//! Service implementation should use UUID, hash(accessToken,Pin,PinStatus.created),
//! or any other opaque identifier that provides equally strong protection against race conditions. \
//!
//! ## Optimizing for speed and connectivity
//! [Here the original](https://ipfs.github.io/pinning-services-api-spec/#section/Provider-hints/Optimizing-for-speed-and-connectivity)
//! and [Rationale](https://ipfs.github.io/pinning-services-api-spec/#section/Provider-hints/Rationale). \
//! Both ends should attempt to preconnect to each other: \
//! - Delegates should always preconnect to origins \
//! - Clients who initiate pin request and also have the pinned data in their own local datastore should preconnect to delegates \
//!
//! **NOTE**: Connections to multiaddrs in origins and delegates arrays should be attempted in best-effort fashion,
//! and dial failure should not fail the pinning operation.
//! When unable to act on explicit provider hints,
//! DHT and other discovery methods should be used as a fallback by a pinning service.
//!

// #![warn(missing_docs)]

mod common;
pub mod errors;
pub mod models;
pub mod vo;
pub mod api;
