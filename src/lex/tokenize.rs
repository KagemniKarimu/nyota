use yake_rust::{get_n_best, Config, ResultItem, StopWords};

type KeyWordList = Vec<ResultItem>;

pub fn extract_keywords(text: &str) -> KeyWordList {
    let config = Config {
        ngrams: 3,
        ..Config::default()
    };
    let ignored = StopWords::predefined("en").unwrap();
    let keywords = get_n_best(10, &text, &ignored, &config);

    keywords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_keyword_extraction() {
        let text = "The quick brown fox jumps over the lazy dog";
        let keywords = extract_keywords(text);

        assert!(!keywords.is_empty(), "Should extract some keywords");

        println!("Basic keywords extracted: {:?}", keywords);

        // Check that stop words are not included
        for keyword in keywords {
            assert!(
                !["the", "over"].contains(&keyword.keyword.as_str()),
                "Should not include stop words"
            );
        }
    }

    #[test]
    fn test_technical_keyword_extraction() {
        let text = "How do I configure the API endpoint for OpenAI GPT-4 model?";
        let keywords = extract_keywords(text);

        println!("Technical keywords extracted: {:?}", keywords);

        // Check for technical terms
        let found_technical_terms: Vec<&str> = keywords
            .iter()
            .map(|k| k.keyword.as_str())
            .filter(|&word| {
                ["api", "endpoint", "openai", "gpt"].contains(&word.to_lowercase().as_str())
            })
            .collect();

        assert!(
            !found_technical_terms.is_empty(),
            "Should find technical terms: {:?}",
            found_technical_terms
        );
    }

    #[test]
    fn test_compound_word_handling() {
        let text = "I need help with smart-contract deployment and cross-chain bridges";
        let keywords = extract_keywords(text);

        println!("Compound keywords extracted: {:?}", keywords);

        // Check handling of compound terms
        let compound_terms: Vec<&str> = keywords
            .iter()
            .map(|k| k.keyword.as_str())
            .filter(|&word| word.contains('-'))
            .collect();

        assert!(
            !compound_terms.is_empty(),
            "Should handle compound terms: {:?}",
            compound_terms
        );
    }

    #[test]
    fn test_keyword_ranking() {
        let text = "Bitcoin Bitcoin Bitcoin is a cryptocurrency cryptocurrency that uses blockchain technology";
        let keywords = extract_keywords(text);

        println!("Ranked keywords: {:?}", keywords);

        // Check that frequency affects ranking
        if let (Some(first), Some(second)) = (keywords.first(), keywords.get(1)) {
            assert!(
                first.score <= second.score,
                "More frequent terms should have lower scores (YAKE scoring)"
            );
        }
    }

    #[test]
    fn test_empty_input() {
        let text = "";
        let keywords = extract_keywords(text);
        assert!(keywords.is_empty(), "Empty text should produce no keywords");
    }

    #[test]
    fn test_single_word() {
        let text = "cryptocurrency";
        let keywords = extract_keywords(text);

        assert!(
            !keywords.is_empty(),
            "Should extract keyword from single word"
        );
        assert_eq!(
            keywords[0].keyword, "cryptocurrency",
            "Should correctly extract single word"
        );
    }

    #[test]
    fn test_special_characters() {
        let text = "Bitcoin! Ethereum? Web3... DeFi: Crypto+";
        let keywords = extract_keywords(text);

        println!("Keywords with special chars: {:?}", keywords);

        // Check that special characters don't interfere with extraction
        let found_terms: Vec<&str> = keywords.iter().map(|k| k.keyword.as_str()).collect();

        assert!(
            found_terms
                .iter()
                .any(|&term| term.eq_ignore_ascii_case("bitcoin")),
            "Should find terms despite punctuation"
        );
    }

    #[test]
    fn test_case_sensitivity() {
        let text1 = "BITCOIN bitcoin Bitcoin BiTcOiN";
        let text2 = "bitcoin bitcoin bitcoin bitcoin";

        let keywords1 = extract_keywords(text1);
        let keywords2 = extract_keywords(text2);

        println!("Case sensitivity comparison:");
        println!("Mixed case keywords: {:?}", keywords1);
        println!("Lower case keywords: {:?}", keywords2);

        // The scores should be similar despite different cases
        assert!(
            (keywords1[0].score - keywords2[0].score).abs() < 0.1,
            "Case should not significantly affect scoring"
        );
    }
}
