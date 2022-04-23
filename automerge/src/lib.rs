#![doc(
    html_logo_url = "https://raw.githubusercontent.com/automerge/automerge-rs/main/img/brandmark.svg"
)]
#![warn(
    missing_debug_implementations,
    // missing_docs, // TODO: add documentation!
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

#[doc(hidden)]
#[macro_export]
macro_rules! log {
     ( $( $t:tt )* ) => {
          {
            use $crate::__log;
            __log!( $( $t )* );
          }
     }
 }

#[cfg(all(feature = "wasm", target_family = "wasm"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log {
     ( $( $t:tt )* ) => {
         web_sys::console::log_1(&format!( $( $t )* ).into());
     }
 }

#[cfg(not(all(feature = "wasm", target_family = "wasm")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log {
     ( $( $t:tt )* ) => {
         println!( $( $t )* );
     }
 }

mod autocommit;
mod automerge;
mod change;
mod clock;
mod columnar;
mod decoding;
mod encoding;
mod error;
mod exid;
mod indexed_cache;
mod keys;
mod keys_at;
mod legacy;
mod op_observer;
mod op_set;
mod op_tree;
mod options;
mod parents;
mod query;
mod range;
mod range_at;
pub mod sync;
pub mod transaction;
mod types;
mod value;
mod values;
mod values_at;
#[cfg(feature = "optree-visualisation")]
mod visualisation;

pub use crate::automerge::Automerge;
pub use autocommit::AutoCommit;
pub use change::Change;
pub use decoding::Error as DecodingError;
pub use decoding::InvalidChangeError;
pub use encoding::Error as EncodingError;
pub use error::AutomergeError;
pub use exid::ExId as ObjId;
pub use keys::Keys;
pub use keys_at::KeysAt;
pub use legacy::Change as ExpandedChange;
pub use op_observer::OpObserver;
pub use op_observer::Patch;
pub use op_observer::VecOpObserver;
pub use options::ApplyOptions;
pub use parents::Parents;
pub use range::Range;
pub use range_at::RangeAt;
pub use types::{ActorId, ChangeHash, ObjType, OpType, Prop};
pub use value::{ScalarValue, Value};
pub use values::Values;
pub use values_at::ValuesAt;

pub const ROOT: ObjId = ObjId::Root;
