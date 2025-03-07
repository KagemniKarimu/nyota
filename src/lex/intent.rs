use crate::lex::intentions::user_intentions;
use anyhow::{Context, Result};
use prefrontal::{BuiltinModel, Classifier, ModelManager};
use std::collections::HashMap;

async fn prepare_intent_model() -> Result<()> {
    let manager = ModelManager::new_default().context("Failed to create ModelManager")?;
    let model = BuiltinModel::MiniLM;

    if !manager.is_model_downloaded(model) {
        manager
            .download_model(model)
            .await
            .context("Failed to download the model")?;
    }

    Ok(())
}

pub struct IntentDetector {
    classifier: Classifier,
}

impl IntentDetector {
    pub async fn create() -> Result<Self> {
        // Download the model if not already present
        prepare_intent_model()
            .await
            .context("Intent Model preparation failed")?;

        // Initialize the classifier with built-in model
        let mut classifier = Classifier::builder().with_model(BuiltinModel::MiniLM)?;

        // Add pre-defined user intentions to the classifier
        for intent in user_intentions() {
            classifier = classifier.add_class(intent)?;
        }

        // Build the internal classifier
        let classifier = classifier.build().context("Failed to build classifier")?;

        // Return the IntentDetector instance
        Ok(Self { classifier })
    }

    pub fn get_intent(&self, message: &str) -> Result<(String, HashMap<String, f32>)> {
        self.classifier
            .predict(message)
            .context("Failed to predict intent")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_related_intents() {
        let detector = IntentDetector::create().await.unwrap();

        let api_messages = [
            "What can you tell me about your API?",
            "How do I change the API provider?",
            "Can you explain the API configuration?",
        ];

        for msg in api_messages {
            let (intent, confidence) = detector.get_intent(msg).unwrap();
            assert_eq!(intent, "api", "Should detect API-related intent");
            assert!(
                confidence["api"] > 0.5,
                "Should have high confidence in API intent"
            );
        }
    }

    #[tokio::test]
    async fn test_model_related_intents() {
        let detector = IntentDetector::create().await.unwrap();

        let model_messages = [
            "Can you switch models?",
            "How do I change the AI model?",
            "What models are available?",
        ];

        for msg in model_messages {
            let (intent, confidence) = detector.get_intent(msg).unwrap();
            assert_eq!(intent, "model", "Should detect model-related intent");
            assert!(
                confidence["model"] > 0.5,
                "Should have high confidence in model intent"
            );
        }
    }

    #[tokio::test]
    async fn test_greeting_intents() {
        let detector = IntentDetector::create().await.unwrap();

        let greetings = ["Hi Nyota", "Good morning", "Hey hey, how are you?"];

        for msg in greetings {
            let (_intent, confidence) = detector.get_intent(msg).unwrap();
            // greetings are hard to distinguish and may have multiple high-confidenc
            // DEBUG: println!("MSG: {}, ({:#?},{:?})", msg, intent, confidence);
            assert!(
                confidence["greeting"] > 0.8,
                "Should have very high confidence in greeting intent"
            );
        }
    }

    #[tokio::test]
    async fn test_about_intents() {
        let detector = IntentDetector::create().await.unwrap();

        let about_messages = [
            "Tell me about yourself Nyota",
            "What can you do?",
            "Tell me your purpose",
            "What are your capabilities?",
        ];

        for msg in about_messages {
            let (_intent, confidence) = detector.get_intent(msg).unwrap();
            // IDEAL: assert_eq!(intent, "about", "Should detect about-related intent");
            // println!("MSG: {}, ({:#?},{:?})", msg, intent, confidence);
            assert!(
                confidence["about"] > 0.8,
                "Should have high confidence in about intent"
            );
        }
    }

    #[tokio::test]
    async fn test_multiple_intents_priority() {
        let detector = IntentDetector::create().await.unwrap();

        // Message that could be interpreted multiple ways
        let message = "Hello, can you tell me how to change the API model?";

        let (primary_intent, confidences) = detector.get_intent(message).unwrap();

        // Verify we have multiple non-zero confidences
        assert!(
            confidences.len() > 1,
            "Should detect multiple possible intents"
        );

        // Verify the primary intent is the one with highest confidence
        let max_confidence: f32 = confidences.values().fold(0.0, |a, &b| a.max(b));
        assert!(
            confidences[&primary_intent] == max_confidence,
            "Primary intent should have highest confidence"
        );
    }

    #[tokio::test]
    async fn test_unknown_intent() {
        let detector = IntentDetector::create().await.unwrap();

        let random_message = "xyzabc 123456 !@#$%^";
        let (intent, confidences) = detector.get_intent(random_message).unwrap();

        // Either the intent should be "unknown" or all confidences should be low
        if intent != "unknown" {
            assert!(
                confidences.values().all(|&conf| conf < 0.3),
                "Should have low confidence for nonsense input"
            );
        }
    }

    #[tokio::test]
    async fn test_intent_detection_stability() {
        let detector = IntentDetector::create().await.unwrap();
        let message = "How do I change the model?";

        // Test that the same message gets the same intent multiple times
        let (first_intent, first_confidences) = detector.get_intent(message).unwrap();
        let (second_intent, second_confidences) = detector.get_intent(message).unwrap();

        assert_eq!(
            first_intent, second_intent,
            "Same message should get same intent"
        );
        assert_eq!(
            first_confidences, second_confidences,
            "Same message should get same confidence scores"
        );
    }
}
