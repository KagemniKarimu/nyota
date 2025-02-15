// New constants for intensity calculation

use crate::lex::sentiment::SentimentState;
use std::fmt;

// New constants for intensity calculation
const MIN_RANGE: f64 = 0.2;
const MAX_PURITY: f64 = 2.0;
const INTERACTION_PLATEAU: f64 = 20.0;

#[derive(Debug)]
pub struct Mood {
    state: MoodState,
    intensity: MoodIntensity,
}

impl Mood {
    pub fn from_sentiment_state(state: &SentimentState) -> Self {
        Self {
            state: MoodState::from_sentiment_state(state),
            intensity: MoodIntensity::from_sentiment_state(state),
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
            x if x >= 0.8 => Self::Extreme,
            x if x >= 0.6 => Self::High,
            x if x >= 0.3 => Self::Medium,
            _ => Self::Low,
        }
    }

    fn get_score(state: &SentimentState) -> f64 {
        // Range factor with dampening
        let range = (state.highest_compound_seen - state.lowest_compound_seen).abs();
        let range_factor = if state.interaction_count == 1 {
            state.compound_affect.abs() // Use raw value for first interaction
        } else if range < MIN_RANGE {
            (state.compound_affect.abs() / MIN_RANGE).powf(1.5)
        } else {
            ((state.compound_affect - state.lowest_compound_seen).abs() / range).powf(1.5)
        };

        // Purity with neutral affect consideration
        let raw_purity = if state.compound_affect >= 0.0 {
            let positive_ratio =
                state.positive_affect / (state.positive_affect + state.negative_affect + 0.1);
            positive_ratio * (1.0 - state.neutral_affect)
        } else {
            let negative_ratio =
                state.negative_affect / (state.positive_affect + state.negative_affect + 0.1);
            negative_ratio * (1.0 - state.neutral_affect)
        };
        let purity = raw_purity.min(1.0);

        // Interaction factor
        let interaction_factor =
            (1.0 + (state.interaction_count as f64).ln() * 0.5) / (1.0 + INTERACTION_PLATEAU.ln());
        let interaction_factor = interaction_factor.min(1.0);

        // Final calculation
        (range_factor * 0.5) + (purity * 0.3) + (interaction_factor * 0.2)
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
    fn from_sentiment_state(state: &SentimentState) -> Self {
        let intensity = MoodIntensity::from_sentiment_state(state);
        let compound = state.compound_affect;

        match (compound, &intensity) {
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
