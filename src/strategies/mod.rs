pub mod accents;
pub mod brackets;
pub mod cjk;
pub mod expansion;
pub mod rtl;
pub mod special_chars;
pub mod unicode_stress;

use crate::cli::StrategyConfig;

/// A single pseudo-translation transformation.
pub trait Strategy {
    fn transform(&self, text: &str) -> String;
}

/// Chains multiple strategies in order.
pub struct StrategyPipeline {
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategyPipeline {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self { strategies }
    }

    /// Apply all strategies in sequence.
    pub fn apply(&self, text: &str) -> String {
        let mut result = text.to_string();
        for strategy in &self.strategies {
            result = strategy.transform(&result);
        }
        result
    }

    /// Returns the number of strategies in the pipeline.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.strategies.len()
    }

    /// Returns true if the pipeline has no strategies.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.strategies.is_empty()
    }
}

/// Build a strategy pipeline from resolved config.
///
/// Order matches the spec:
/// 1. Accents
/// 2. CJK
/// 3. Special chars
/// 4. Unicode stress
/// 5. Expansion
/// 6. RTL
/// 7. Brackets
pub fn build_pipeline(config: &StrategyConfig) -> StrategyPipeline {
    let mut strategies: Vec<Box<dyn Strategy>> = Vec::new();

    if config.accents {
        strategies.push(Box::new(accents::AccentStrategy));
    }
    if config.cjk {
        strategies.push(Box::new(cjk::CjkStrategy));
    }
    if config.special_chars {
        strategies.push(Box::new(special_chars::SpecialCharsStrategy));
    }
    if config.unicode_stress {
        strategies.push(Box::new(unicode_stress::UnicodeStressStrategy));
    }
    if let Some(ratio) = config.expansion {
        strategies.push(Box::new(expansion::ExpansionStrategy::new(ratio)));
    }
    if config.rtl {
        strategies.push(Box::new(rtl::RtlStrategy));
    }
    if config.brackets {
        strategies.push(Box::new(brackets::BracketStrategy));
    }

    StrategyPipeline::new(strategies)
}
