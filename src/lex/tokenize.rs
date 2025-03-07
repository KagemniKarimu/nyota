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
