// New constants for intensity calculation

use crate::lex::sentiment::Sentiment;
use crate::lex::sentiment::SentimentState;

use std::fmt;

use super::sentiment::AffectScore;

// New constants for intensity calculation
const MAX_SENTIMENT_FACTOR: f64 = 1.0;
const MAX_PURITY_FACTOR: f64 = 1.0;
const MAX_FAMILIARITY_FACTOR: f64 = 1.0;

const INTERACTION_INFLUENCE_STRENGTH: f64 = 0.75;
const INTERACTION_MATURITY_POINT: f64 = 25.0;
const MINIMUM_SENTIMENT_RANGE: f64 = 0.2; // Minimum significant range of emotional variation
const EMOTIONAL_AMPLIFICATION_FACTOR: f64 = 1.5; // Power factor to amplify stronger emotions
const FIRST_INTERACTION_COUNTED: usize = 1;

const PURITY_SMOOTHING_FACTOR: f64 = 0.1; // Smoothing factor for purity calculation
const PURITY_POSITIVITY_THRESHHOLD: f64 = 0.0; // Threshold for positive affect in purity calculation
const MAXIMUM_NORMALIZED_NEUTRAL_AFFECT: f64 = 1.0; // Maximum normalized neutral affect for purity calculation

const SENTIMENT_RANGE_FACTOR_WEIGHT: f64 = 0.1;
const SENTIMENT_PURITY_FACTOR_WEIGHT: f64 = 0.3;
const INTERACTION_FAMILIARITY_FACTOR_WEIGHT: f64 = 0.6;

const MOOD_INTENSITY_HIGH_THRESHHOLD: f64 = 0.8;
const MOOD_INTENSITY_MEDIUM_THRESHHOLD: f64 = 0.6;
const MOOD_INTENSITY_LOW_THRESHHOLD: f64 = 0.3;

#[derive(Debug)]
pub struct Mood {
    state: MoodState,
    intensity: MoodIntensity,
}

impl Mood {
    pub async fn from_sentiment_state(sentiment_state: &SentimentState) -> Self {
        Self {
            state: MoodState::from_sentiment_state(sentiment_state).await,
            intensity: MoodIntensity::from_sentiment_state(sentiment_state),
        }
    }
}

impl fmt::Display for Mood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // This uses the Debug impl we already have to translate Mood to a String
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoodIntensity {
    Low,
    Medium,
    High,
    Extreme,
}

impl MoodIntensity {
    fn from_sentiment_state(state: &SentimentState) -> Self {
        match MoodIntensity::get_score(state) {
            x if x >= MOOD_INTENSITY_HIGH_THRESHHOLD => Self::Extreme,
            x if x >= MOOD_INTENSITY_MEDIUM_THRESHHOLD => Self::High,
            x if x >= MOOD_INTENSITY_LOW_THRESHHOLD => Self::Medium,
            _ => Self::Low,
        }
    }

    fn get_score(state: &SentimentState) -> f64 {
        let sentiment_range_factor = MoodIntensity::calculate_sentiment_range_factor(state);
        let sentiment_purity_factor = MoodIntensity::calculate_sentiment_purity_factor(state);
        let interaction_familiarity_factor = MoodIntensity::calculate_familiarity_factor(state);

        // Final calculation
        (sentiment_range_factor * SENTIMENT_RANGE_FACTOR_WEIGHT)
            + (sentiment_purity_factor * SENTIMENT_PURITY_FACTOR_WEIGHT)
            + (interaction_familiarity_factor * INTERACTION_FAMILIARITY_FACTOR_WEIGHT)
    }

    fn normalize_affect(affect: AffectScore, total_affect: AffectScore) -> f64 {
        if total_affect == 0.0 {
            0.0
        } else {
            affect / total_affect
        }
    }

    fn calculate_sentiment_range_factor(state: &SentimentState) -> f64 {
        let sentiment_range = (state.highest_compound_seen - state.lowest_compound_seen).abs();
        let sentiment_range_factor = if state.interaction_count == FIRST_INTERACTION_COUNTED {
            state.compound_affect.abs() // Use raw value for first interaction
        } else if sentiment_range < MINIMUM_SENTIMENT_RANGE {
            (state.compound_affect.abs() / MINIMUM_SENTIMENT_RANGE)
                .powf(EMOTIONAL_AMPLIFICATION_FACTOR)
        } else {
            ((state.compound_affect - state.lowest_compound_seen).abs() / sentiment_range)
                .powf(EMOTIONAL_AMPLIFICATION_FACTOR)
        };

        sentiment_range_factor.min(MAX_SENTIMENT_FACTOR)
    }

    fn calculate_sentiment_purity_factor(state: &SentimentState) -> f64 {
        let total_affect = state.positive_affect + state.negative_affect + state.neutral_affect;
        let normalized_neutral_affect =
            MoodIntensity::normalize_affect(state.neutral_affect, total_affect);
        let emotional_portion = MAXIMUM_NORMALIZED_NEUTRAL_AFFECT - normalized_neutral_affect;

        let sentiment_purity = if state.compound_affect >= PURITY_POSITIVITY_THRESHHOLD {
            let positive_affect_ratio = state.positive_affect
                / (state.positive_affect + state.negative_affect + PURITY_SMOOTHING_FACTOR);
            positive_affect_ratio * emotional_portion
        } else {
            let negative_affect_ratio = state.negative_affect
                / (state.positive_affect + state.negative_affect + PURITY_SMOOTHING_FACTOR);
            negative_affect_ratio * emotional_portion
        };

        sentiment_purity.min(MAX_PURITY_FACTOR)
    }

    fn calculate_familiarity_factor(state: &SentimentState) -> f64 {
        let familiarity_factor = (1.0
            + (state.interaction_count as f64).ln() * INTERACTION_INFLUENCE_STRENGTH)
            / (1.0 + INTERACTION_MATURITY_POINT.ln());

        familiarity_factor.min(MAX_FAMILIARITY_FACTOR)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoodState {
    // Extreme Intensity Positive
    Ecstatic,    // Pure joy
    Exhilarated, // High energy positive
    Euphoric,    // Intense happiness

    // High Intensity Positive
    Elated,       // Strong happiness
    Enthusiastic, // High energy
    Excited,      // Strong positive activation

    // Medium Intensity Positive
    Happy,    // Clear positive
    Cheerful, // Light positive
    Pleased,  // Moderate satisfaction

    // Low Intensity Positive
    Content,     // Quiet satisfaction
    Satisfied,   // Mild positive
    Comfortable, // Gentle ease

    // Neutral
    Intrigued, // Mild interest
    Calm,      // Balance
    Pensive,   // Quiet reflection

    // Low Intensity Negative
    Unsettled, // Mild discomfort
    Uneasy,    // Slight worry
    Concerned, // Gentle anxiety

    // Medium Intensity Negative
    Frustrated, // Clear negativity
    Anxious,    // Notable worry
    Distressed, // Definite trouble

    // High Intensity Negative
    Angry,   // Strong negativity
    Enraged, // Intense anger
    Furious, // High energy negative

    // Extreme Intensity Negative
    Despairing, // Deep sadness
    Devastated, // Complete negativity
    Anguished,  // Overwhelming distress
}

impl MoodState {
    async fn from_sentiment_state(state: &SentimentState) -> Self {
        let total_emotion = state.compound_affect;
        let intensity = MoodIntensity::from_sentiment_state(state);

        match (total_emotion, &intensity) {
            // Extreme Positive Intensity
            (c, MoodIntensity::Extreme) if c >= 0.7 => Self::Ecstatic,
            (c, MoodIntensity::Extreme) if c >= 0.5 => Self::Exhilarated,
            (c, MoodIntensity::Extreme) if c >= 0.3 => Self::Euphoric,

            // High Positive Intensity
            (c, MoodIntensity::High) if c >= 0.7 => Self::Elated,
            (c, MoodIntensity::High) if c >= 0.5 => Self::Enthusiastic,
            (c, MoodIntensity::High) if c >= 0.3 => Self::Excited,

            // Medium Positive Intensity
            (c, MoodIntensity::Medium) if c >= 0.6 => Self::Happy,
            (c, MoodIntensity::Medium) if c >= 0.4 => Self::Cheerful,
            (c, MoodIntensity::Medium) if c >= 0.2 => Self::Pleased,

            // Low Positive Intensity
            (c, MoodIntensity::Low) if c >= 0.6 => Self::Content,
            (c, MoodIntensity::Low) if c >= 0.4 => Self::Satisfied,
            (c, MoodIntensity::Low) if c >= 0.2 => Self::Comfortable,

            // Extreme Negative Intensity
            (c, MoodIntensity::Extreme) if c <= -0.7 => Self::Anguished,
            (c, MoodIntensity::Extreme) if c <= -0.5 => Self::Devastated,
            (c, MoodIntensity::Extreme) if c <= -0.3 => Self::Despairing,

            // High Negative Intensity
            (c, MoodIntensity::High) if c <= -0.7 => Self::Furious,
            (c, MoodIntensity::High) if c <= -0.5 => Self::Enraged,
            (c, MoodIntensity::High) if c <= -0.3 => Self::Angry,

            // Medium Negative Intensity
            (c, MoodIntensity::Medium) if c <= -0.6 => Self::Distressed,
            (c, MoodIntensity::Medium) if c <= -0.4 => Self::Anxious,
            (c, MoodIntensity::Medium) if c <= -0.2 => Self::Frustrated,

            // Low Negative Intensity
            (c, MoodIntensity::Low) if c <= -0.6 => Self::Concerned,
            (c, MoodIntensity::Low) if c <= -0.4 => Self::Uneasy,
            (c, MoodIntensity::Low) if c <= -0.2 => Self::Unsettled,

            // Neutral states (at any intensity)
            (c, _) if c > 0.1 => Self::Intrigued,
            (c, _) if c < -0.1 => Self::Pensive,
            _ => Self::Calm,
        }
    }
}
