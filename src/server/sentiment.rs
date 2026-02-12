/// Simple keyword-based sentiment analyzer.
/// Returns: 1 = negative, 2 = neutral, 3 = positive.
pub fn analyze_sentiment(text: &str) -> u8 {
    let lower = text.to_lowercase();

    let negative_words = [
        "bad", "terrible", "awful", "horrible", "worst", "disgusting",
        "rude", "cold", "stale", "overpriced", "slow", "dirty",
        "unacceptable", "tasteless", "inedible", "disappointing",
        "poor", "mediocre", "gross", "nasty", "hate", "angry",
        "complaint", "never again", "waste", "burnt", "raw",
        "food poisoning", "sick", "unhygienic", "cockroach", "fly",
    ];

    let positive_words = [
        "good", "great", "excellent", "amazing", "wonderful", "fantastic",
        "delicious", "fresh", "friendly", "perfect", "love", "best",
        "outstanding", "superb", "recommend", "beautiful", "cozy",
        "elegant", "exquisite", "refined", "impeccable", "divine",
        "scrumptious", "heavenly", "brilliant", "stellar", "lovely",
        "charming", "pleasant", "attentive", "exceptional", "top-notch",
    ];

    let neg_count = negative_words.iter().filter(|w| lower.contains(*w)).count();
    let pos_count = positive_words.iter().filter(|w| lower.contains(*w)).count();

    if neg_count > pos_count {
        1 // negative
    } else if pos_count > neg_count {
        3 // positive
    } else {
        2 // neutral
    }
}

/// Compute a scoring value from 0–100 based on keyword density.
pub fn compute_scoring(text: &str) -> u8 {
    let lower = text.to_lowercase();
    let word_count = lower.split_whitespace().count().max(1) as f64;

    let negative_words = [
        "bad", "terrible", "awful", "horrible", "worst", "disgusting",
        "rude", "cold", "stale", "overpriced", "slow", "dirty",
        "hate", "angry", "complaint", "waste",
    ];
    let positive_words = [
        "good", "great", "excellent", "amazing", "wonderful", "fantastic",
        "delicious", "fresh", "friendly", "perfect", "love", "best",
        "recommend", "beautiful",
    ];

    let neg_count = negative_words.iter().filter(|w| lower.contains(*w)).count() as f64;
    let pos_count = positive_words.iter().filter(|w| lower.contains(*w)).count() as f64;

    // Score: 50 = neutral, <50 = negative leaning, >50 = positive leaning
    let ratio = (pos_count - neg_count) / word_count;
    let score = (50.0 + ratio * 100.0).clamp(0.0, 100.0) as u8;
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_sentiment() {
        assert_eq!(analyze_sentiment("This food was terrible and disgusting"), 1);
    }

    #[test]
    fn test_positive_sentiment() {
        assert_eq!(analyze_sentiment("The food was excellent and amazing!"), 3);
    }

    #[test]
    fn test_neutral_sentiment() {
        assert_eq!(analyze_sentiment("I had dinner here last night"), 2);
    }

    #[test]
    fn test_scoring() {
        let score = compute_scoring("terrible food, very bad");
        assert!(score < 50);
        let score = compute_scoring("excellent and amazing food");
        assert!(score > 50);
    }
}
