use serde::{Deserialize, Serialize};

/// The level of detail requested for a query extraction.
///
/// Ordered from cheapest (fewest tokens) to most expensive.
/// Default is `IdsOnly` to be maximally token-conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ExtractionIntent {
    /// Only check if the item exists. Cheapest possible query.
    Exists,
    /// Return only identifiers. Default and very cheap.
    #[default]
    IdsOnly,
    /// Return a compact summary (key fields only).
    Summary,
    /// Return specific named fields.
    Fields,
    /// Return the full object. Most expensive.
    Full,
}

impl ExtractionIntent {
    /// Estimated relative token cost for this intent level.
    /// Returns a multiplier relative to `IdsOnly` (which is 1).
    pub fn estimated_tokens(&self) -> u64 {
        match self {
            ExtractionIntent::Exists => 1,
            ExtractionIntent::IdsOnly => 2,
            ExtractionIntent::Summary => 10,
            ExtractionIntent::Fields => 25,
            ExtractionIntent::Full => 100,
        }
    }

    /// Whether this intent requests the full payload.
    pub fn is_full(&self) -> bool {
        matches!(self, ExtractionIntent::Full)
    }

    /// Whether this is a minimal (token-conservative) intent.
    pub fn is_minimal(&self) -> bool {
        matches!(self, ExtractionIntent::Exists | ExtractionIntent::IdsOnly)
    }
}

/// Trait for types that can have an extraction intent applied to scope their output.
pub trait Scopeable {
    /// The scoped output type.
    type Output;

    /// Apply the given intent to produce a scoped result.
    fn apply_intent(&self, intent: ExtractionIntent) -> ScopedResult<Self::Output>;
}

/// The result of applying an extraction intent to scope a query response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopedResult<T> {
    /// The item exists (response to `Exists` intent).
    Exists(bool),
    /// Only identifiers are returned.
    IdsOnly(Vec<String>),
    /// A compact summary.
    Summary(String),
    /// Selected fields.
    Fields(T),
    /// The full object.
    Full(T),
}

impl<T> ScopedResult<T> {
    /// Estimated token cost of this result.
    pub fn estimated_tokens(&self) -> u64 {
        match self {
            ScopedResult::Exists(_) => 1,
            ScopedResult::IdsOnly(ids) => ids.len() as u64 * 2,
            ScopedResult::Summary(_) => 10,
            ScopedResult::Fields(_) => 25,
            ScopedResult::Full(_) => 100,
        }
    }
}

/// Apply an extraction intent to a full-size data vector, returning the
/// scoped result. This is the primary entry point for token conservation
/// at the query layer.
pub fn apply_intent<T: Clone + Serialize>(
    data: &[T],
    intent: ExtractionIntent,
    id_extractor: impl Fn(&T) -> String,
    summary_extractor: impl Fn(&[T]) -> String,
) -> ScopedResult<Vec<T>> {
    match intent {
        ExtractionIntent::Exists => ScopedResult::Exists(!data.is_empty()),
        ExtractionIntent::IdsOnly => {
            ScopedResult::IdsOnly(data.iter().map(&id_extractor).collect())
        }
        ExtractionIntent::Summary => ScopedResult::Summary(summary_extractor(data)),
        ExtractionIntent::Fields => ScopedResult::Fields(data.to_vec()),
        ExtractionIntent::Full => ScopedResult::Full(data.to_vec()),
    }
}
