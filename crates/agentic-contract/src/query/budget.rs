use serde::{Deserialize, Serialize};

/// A token budget that tracks spending against a maximum allowance.
///
/// Used to enforce token limits across queries, preventing runaway
/// token consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBudget {
    /// Maximum tokens allowed.
    max_tokens: u64,
    /// Tokens consumed so far.
    used_tokens: u64,
}

impl TokenBudget {
    /// Create a new token budget with the given maximum.
    pub fn new(max_tokens: u64) -> Self {
        Self {
            max_tokens,
            used_tokens: 0,
        }
    }

    /// Spend tokens from the budget. Returns `true` if the spend was
    /// within budget, `false` if it would exceed the budget (spend still occurs).
    pub fn spend(&mut self, tokens: u64) -> bool {
        let within = self.can_afford(tokens);
        self.used_tokens = self.used_tokens.saturating_add(tokens);
        within
    }

    /// Try to spend tokens, only succeeding if within budget.
    /// Returns `true` if the spend succeeded, `false` if it would exceed budget.
    pub fn try_spend(&mut self, tokens: u64) -> bool {
        if self.can_afford(tokens) {
            self.used_tokens = self.used_tokens.saturating_add(tokens);
            true
        } else {
            false
        }
    }

    /// Get the number of remaining tokens.
    pub fn remaining(&self) -> u64 {
        self.max_tokens.saturating_sub(self.used_tokens)
    }

    /// Check if the budget is fully exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.used_tokens >= self.max_tokens
    }

    /// Check if the budget can afford a given number of tokens.
    pub fn can_afford(&self, tokens: u64) -> bool {
        self.used_tokens.saturating_add(tokens) <= self.max_tokens
    }

    /// Get the utilization ratio (0.0 to 1.0+).
    pub fn utilization(&self) -> f64 {
        if self.max_tokens == 0 {
            return if self.used_tokens > 0 { 1.0 } else { 0.0 };
        }
        self.used_tokens as f64 / self.max_tokens as f64
    }

    /// Get the maximum token allowance.
    pub fn max_tokens(&self) -> u64 {
        self.max_tokens
    }

    /// Get the number of tokens used so far.
    pub fn used_tokens(&self) -> u64 {
        self.used_tokens
    }

    /// Reset the budget to zero usage.
    pub fn reset(&mut self) {
        self.used_tokens = 0;
    }
}
