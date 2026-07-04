use core_macros::nested;

/// One indexer a filter is attached to. autobrr indexers aren't managed by
/// configuratarr (they're defined server-side), so this just identifies an
/// existing one by id / identifier.
#[nested(case = snake)]
pub struct FilterIndexer {
    /// Server indexer id.
    pub id: Option<i32>,
    /// Indexer display name.
    pub name: Option<String>,
    /// Indexer identifier (e.g. `torrentleech`).
    pub identifier: Option<String>,
}
