use serde::{Deserialize, Serialize};

/// A cursor-based page of results.
///
/// Uses opaque string cursors for stable, token-efficient pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPage<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// The cursor pointing to the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether there are more results after this page.
    pub has_more: bool,
    /// Total number of items across all pages (if known).
    pub total: Option<usize>,
}

impl<T: Clone> CursorPage<T> {
    /// Create a page from a slice using cursor-based pagination.
    ///
    /// The `cursor` is a 0-based offset encoded as a string.
    /// If `cursor` is `None`, starts from the beginning.
    /// `limit` controls how many items to include in this page.
    pub fn from_slice(data: &[T], cursor: Option<&str>, limit: usize) -> Self {
        let offset = cursor
            .and_then(|c| c.parse::<usize>().ok())
            .unwrap_or(0);

        let total_len = data.len();

        if offset >= total_len {
            return Self {
                items: Vec::new(),
                next_cursor: None,
                has_more: false,
                total: Some(total_len),
            };
        }

        let end = (offset + limit).min(total_len);
        let items = data[offset..end].to_vec();
        let has_more = end < total_len;
        let next_cursor = if has_more {
            Some(end.to_string())
        } else {
            None
        };

        Self {
            items,
            next_cursor,
            has_more,
            total: Some(total_len),
        }
    }

    /// Create an empty page.
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            next_cursor: None,
            has_more: false,
            total: Some(0),
        }
    }

    /// Number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether this page is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Map the items in this page to a different type.
    pub fn map<U: Clone>(self, f: impl Fn(T) -> U) -> CursorPage<U> {
        CursorPage {
            items: self.items.into_iter().map(f).collect(),
            next_cursor: self.next_cursor,
            has_more: self.has_more,
            total: self.total,
        }
    }
}
