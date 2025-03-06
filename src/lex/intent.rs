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
