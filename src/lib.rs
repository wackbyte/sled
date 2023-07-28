// TODO for write batches, find new way of dropping LeafGuard types that avoids
//      calling mark_access_and_evict while any mutex is held for leaves. must
//      first drop all locks then do mark access on each.
// TODO document marble not to use too high pid otherwise recovery will lock
//      on essentially infinite recovery
// TODO marble maintenance w/ speculative write followed by CAS in pt
//      maybe with global writer lock that controls flushers too
// TODO re-enable paging out
// TODO write background flusher
// TODO max key and value sizes w/ corresponding heap
mod db;
mod flush_epoch;

pub use crate::db::{open_default, Config, Db};
pub use marble::InlineArray;

use crate::flush_epoch::{FlushEpoch, FlushEpochGuard};

/// Compare and swap result.
///
/// It returns `Ok(Ok(()))` if operation finishes successfully and
///     - `Ok(Err(CompareAndSwapError(current, proposed)))` if operation failed
///       to setup a new value. `CompareAndSwapError` contains current and
///       proposed values.
///     - `Err(Error::Unsupported)` if the database is opened in read-only mode.
///       otherwise.
pub type CompareAndSwapResult = std::io::Result<
    std::result::Result<CompareAndSwapSuccess, CompareAndSwapError>,
>;

/// Compare and swap error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompareAndSwapError {
    /// The current value which caused your CAS to fail.
    pub current: Option<InlineArray>,
    /// Returned value that was proposed unsuccessfully.
    pub proposed: Option<InlineArray>,
}

/// Compare and swap success.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompareAndSwapSuccess {
    /// The current value which was successfully installed.
    pub new_value: Option<InlineArray>,
    /// Returned value that was previously stored.
    pub previous_value: Option<InlineArray>,
}

impl std::fmt::Display for CompareAndSwapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compare and swap conflict")
    }
}

impl std::error::Error for CompareAndSwapError {}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct NodeId(u64);
