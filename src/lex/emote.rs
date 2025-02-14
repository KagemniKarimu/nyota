use clap::Error;
use vader_sentimental::{SentimentIntensity, SentimentIntensityAnalyzer};

const DEFAULT_AGGREGATE_AFFECT: f64 = 1.0;
const DEFAULT_POSITIVE_AFFECT: f64 = 1.0;
const DEFAULT_NEGATIVE_AFFECT: f64 = 0.0;
const DEFAULT_NEUTRAL_AFFECT: f64 = 0.2;

struct CollectedSentiment {
    aggregate_affect: f64,
    positive_affect: f64,
    negative_affect: f64,
    neutral_affect: f64,
}

impl CollectedSentiment {
    fn new() -> Self {
        Self {
            aggregate_affect: DEFAULT_AGGREGATE_AFFECT,
            positive_affect: DEFAULT_POSITIVE_AFFECT,
            negative_affect: DEFAULT_NEGATIVE_AFFECT,
            neutral_affect: DEFAULT_NEUTRAL_AFFECT,
        }
    }

    fn add_feeling(&mut self, sentiment: SentimentIntensity) -> Result<(), Error> {
        Self {
            aggregate_affect: &self.aggregate_affect + sentiment.compound,
            positive_affect: &self.positive_affect + sentiment.pos,
            negative_affect: &self.negative_affect + sentiment.neg,
            neutral_affect: &self.neutral_affect + sentiment.neu,
        };
        Ok(())
    }

    fn forget_feelings(&self) -> Result<(), Error> {
        Self {
            aggregate_affect: DEFAULT_AGGREGATE_AFFECT,
            positive_affect: DEFAULT_POSITIVE_AFFECT,
            negative_affect: DEFAULT_NEGATIVE_AFFECT,
            neutral_affect: DEFAULT_NEUTRAL_AFFECT,
        };
        Ok(())
    }
}

fn analyze_sentiment(message: &str) -> SentimentIntensity {
    let analyzer = SentimentIntensityAnalyzer::new();
    analyzer.polarity_scores(message)
}

//fn construct_sentiment(intensity_output: SentimentIntensity) -> Sentiment {
//   Sentiment {
//       compound: intensity_output.compound,
//       pos: intensity_output.pos,
//       neg: intensity_output.neg,
//       neu: intensity_output.neu,
//   }
//
//
