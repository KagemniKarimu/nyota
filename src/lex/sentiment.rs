//! The `sentiment` module provides a way to track the sentiment of a session.
//! It uses the `vader_sentimental` crate to analyze and accumulate the sentiment of messages.
//! The `Sentiment` struct is used to track the sentiment of a session.
//! The sentiment values are stored in a `SentimentState` struct, which includes the compound affect, positive affect, negative affect, neutral affect, and interaction count of all processed messages.
//! The idea is that simple interactions accumulate over time to build a set of feelings.

use anyhow::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use vader_sentimental::{SentimentIntensity, SentimentIntensityAnalyzer};

const DEFAULT_COMPOUND_AFFECT: f64 = 0.0;
const DEFAULT_POSITIVE_AFFECT: f64 = 0.0;
const DEFAULT_NEGATIVE_AFFECT: f64 = 0.0;
const DEFAULT_NEUTRAL_AFFECT: f64 = 0.0;

const MINIMUM_COMPOUND_AFFECT: f64 = -1.0;
const MAXIMUM_COMPOUND_AFFECT: f64 = 1.0;

/// The `Sentiment` struct is used to track the sentiment of a session.
/// It uses the `SentimentIntensityAnalyzer` from the `vader_sentimental` crate to analyze and accumulate the sentiment of messages.
/// The sentiment values are stored in a `SentimentState` struct, which includes the compound affect, positive affect, negative affect, neutral affect, and interaction count of all processed messages.
#[derive(Debug)]
pub struct Sentiment<'a> {
    state: Arc<Mutex<SentimentState>>,
    analyzer: SentimentIntensityAnalyzer<'a>,
}

/// The `SentimentState` struct is used to store the sentiment values of a session.
/// It includes the compound affect, positive affect, negative affect, neutral affect, and interaction count of all processed messages.
/// It also includes the lowest and highest compound values seen so far, which are used to normalize the compound affect value.
#[derive(Debug)]
pub struct SentimentState {
    pub compound_affect: f64,
    pub positive_affect: f64,
    pub negative_affect: f64,
    pub neutral_affect: f64,
    pub interaction_count: usize,
    pub lowest_compound_seen: f64,
    pub highest_compound_seen: f64,
}

/// Implementation of the `Sentiment` struct.
/// The three public interfaces are `new`, `forget_feelings`, `get_feelings`, and `process_emotion`.
/// The `new` method creates a new `Sentiment` struct with default values.
/// The `forget_feelings` method resets existing sentiment state to default values.
/// The `get_feelings` method returns the current sentiment state.
/// The `process_emotion` method processes the sentiment of a message and updates the internal sentiment state.
impl<'a> Sentiment<'a> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(SentimentState {
                compound_affect: DEFAULT_COMPOUND_AFFECT,
                positive_affect: DEFAULT_POSITIVE_AFFECT,
                negative_affect: DEFAULT_NEGATIVE_AFFECT,
                neutral_affect: DEFAULT_NEUTRAL_AFFECT,
                interaction_count: 0,
                lowest_compound_seen: DEFAULT_COMPOUND_AFFECT,
                highest_compound_seen: DEFAULT_COMPOUND_AFFECT,
            })),
            analyzer: SentimentIntensityAnalyzer::new(),
        }
    }

    /// Add a new sentiment to the sentiment state.
    /// This method increments the interaction count and updates the sentiment values based on the most recent feeling in light of historical feelings.
    /// The sentiment values are weighted averages of the sentiment values seen so far.
    async fn add_feeling(&self, sentiment: SentimentIntensity) -> Result<(), Error> {
        let mut state = self.state.lock().await;

        // Track raw sentiment bounds for normalization
        // This is useful for normalizing the compound affect based on values seen so far
        state.lowest_compound_seen = state.lowest_compound_seen.min(sentiment.compound);
        state.highest_compound_seen = state.highest_compound_seen.max(sentiment.compound);

        // Calculate the weighted average of the sentiment values
        // This is to smooth out the sentiment values over time
        let weight = 1.0 / (state.interaction_count + 1) as f64;
        let old_weight = 1.0 - weight;

        // Update the sentiment state with the new values
        let new_compound = (old_weight * state.compound_affect) + (weight * sentiment.compound);
        state.compound_affect = new_compound;
        state.positive_affect = (old_weight * state.positive_affect) + (weight * sentiment.pos);
        state.negative_affect = (old_weight * state.negative_affect) + (weight * sentiment.neg);
        state.neutral_affect = (old_weight * state.neutral_affect) + (weight * sentiment.neu);

        // Increment the interaction count
        state.interaction_count += 1;
        Ok(())
    }

    /// Forget all feelings and reset the sentiment state to default values.
    /// This is useful for resetting the sentiment state after a conversation has ended.
    pub async fn forget_feelings(&self) -> Result<(), Error> {
        let mut state = self.state.lock().await;
        // Reset the sentiment state to default values
        state.compound_affect = DEFAULT_COMPOUND_AFFECT;
        state.positive_affect = DEFAULT_POSITIVE_AFFECT;
        state.negative_affect = DEFAULT_NEGATIVE_AFFECT;
        state.neutral_affect = DEFAULT_NEUTRAL_AFFECT;
        // Reset the interaction count to 0
        state.interaction_count = 0;
        // Reset the lowest and highest compound values to the default
        state.lowest_compound_seen = DEFAULT_COMPOUND_AFFECT;
        state.highest_compound_seen = DEFAULT_COMPOUND_AFFECT;
        Ok(())
    }

    /// Get the current sentiment state. This includes the compound affect, positive affect, negative affect, neutral affect, and interaction count.
    /// The compound affect is normalized to a range between -1 and 1. The lowest and highest compound values are used to normalize the value.
    pub async fn get_feelings(&self) -> Result<SentimentState, Error> {
        let state = self.state.lock().await;
        Ok(SentimentState {
            compound_affect: self.normalize_compound(
                state.compound_affect,
                state.lowest_compound_seen,
                state.highest_compound_seen,
            ),
            positive_affect: state.positive_affect,
            negative_affect: state.negative_affect,
            neutral_affect: state.neutral_affect,
            interaction_count: state.interaction_count,
            lowest_compound_seen: state.lowest_compound_seen,
            highest_compound_seen: state.highest_compound_seen,
        })
    }

    /// Map the sentiment of a message to a SentimentIntensity struct.
    fn map_sentiment(&self, message: &str) -> SentimentIntensity {
        self.analyzer.polarity_scores(message)
    }

    /// Normalize the compound affect value to a range between 0 and 1.
    /// This is useful for visualizing the compound affect in a normalized range.
    /// The minimum and maximum values are used to normalize the value.
    /// If the minimum and maximum values are the same, the value is returned as is.
    /// The normalized value is returned.
    fn normalize_compound(&self, value: f64, min: f64, max: f64) -> f64 {
        // If we have no range yet, or value is already within the range, return as is
        if (max - min).abs() < f64::EPSILON
            || (value >= MINIMUM_COMPOUND_AFFECT && value <= MAXIMUM_COMPOUND_AFFECT)
        {
            return value;
        }

        // If the value is outside the range, normalize it
        if max > MAXIMUM_COMPOUND_AFFECT || min < MINIMUM_COMPOUND_AFFECT {
            (value - min) / (max - min) * (MAXIMUM_COMPOUND_AFFECT - MINIMUM_COMPOUND_AFFECT)
                + MINIMUM_COMPOUND_AFFECT
        } else {
            value
        }
    }

    /// Interpret interpolated emojis in a message into words and return a string with their descriptions interpolated.
    pub fn interpret_emojis(&self, message: &str) -> String {
        self.analyzer.append_emoji_descriptions(message)
    }

    /// Process the sentiment of a message and update the internal Sentiment.
    pub async fn process_emotion(&self, message: &str) -> Result<(), Error> {
        let sentiment = self.map_sentiment(message);
        self.add_feeling(sentiment).await?;
        Ok(())
    }
}
