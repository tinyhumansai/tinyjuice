//! Dependency-free BM25 scorer.

use std::collections::{HashMap, HashSet};

const K1: f32 = 1.2;
const B: f32 = 0.75;

#[derive(Debug, Clone, PartialEq)]
pub struct Bm25DocumentScore {
    pub index: usize,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct Bm25Corpus {
    docs: Vec<Vec<String>>,
    frequencies: Vec<HashMap<String, usize>>,
    document_frequency: HashMap<String, usize>,
    average_len: f32,
}

impl Bm25Corpus {
    pub fn new<'a>(documents: impl IntoIterator<Item = &'a str>) -> Self {
        let docs: Vec<Vec<String>> = documents.into_iter().map(tokenize).collect();
        Self::from_tokenized(docs)
    }

    pub fn from_tokenized(docs: Vec<Vec<String>>) -> Self {
        let mut frequencies = Vec::with_capacity(docs.len());
        let mut document_frequency: HashMap<String, usize> = HashMap::new();
        let mut total_len = 0usize;

        for doc in &docs {
            total_len += doc.len();
            let mut counts: HashMap<String, usize> = HashMap::new();
            for token in doc {
                *counts.entry(token.clone()).or_insert(0) += 1;
            }
            let unique: HashSet<&String> = doc.iter().collect();
            for token in unique {
                *document_frequency.entry(token.clone()).or_insert(0) += 1;
            }
            frequencies.push(counts);
        }

        let average_len = if docs.is_empty() {
            0.0
        } else {
            total_len as f32 / docs.len() as f32
        };

        Self {
            docs,
            frequencies,
            document_frequency,
            average_len,
        }
    }

    pub fn score(&self, query: &str, index: usize) -> f32 {
        let Some(doc) = self.docs.get(index) else {
            return 0.0;
        };
        if doc.is_empty() || self.average_len <= f32::EPSILON {
            return 0.0;
        }

        let query_tokens = tokenize(query);
        if query_tokens.is_empty() {
            return 0.0;
        }

        let mut score = 0.0f32;
        let doc_len = doc.len() as f32;
        let doc_count = self.docs.len() as f32;
        let counts = &self.frequencies[index];
        let unique_query: HashSet<String> = query_tokens.into_iter().collect();

        for token in unique_query {
            let Some(&tf_count) = counts.get(&token) else {
                continue;
            };
            let df = *self.document_frequency.get(&token).unwrap_or(&0) as f32;
            let idf = ((doc_count - df + 0.5) / (df + 0.5) + 1.0).ln();
            let tf = tf_count as f32;
            let denom = tf + K1 * (1.0 - B + B * (doc_len / self.average_len));
            score += idf * (tf * (K1 + 1.0)) / denom;
        }

        score + exact_identifier_boost(query, doc)
    }

    pub fn score_all(&self, query: &str) -> Vec<Bm25DocumentScore> {
        (0..self.docs.len())
            .map(|index| Bm25DocumentScore {
                index,
                score: self.score(query, index),
            })
            .collect()
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' || ch == '@' {
            current.push(ch.to_ascii_lowercase());
        } else {
            push_token(&mut tokens, &mut current);
        }
    }
    push_token(&mut tokens, &mut current);
    tokens
}

fn push_token(tokens: &mut Vec<String>, current: &mut String) {
    let token = current.trim_matches(|c: char| c == '-' || c == '.' || c == '@');
    if token.len() >= 2 {
        tokens.push(token.to_string());
    }
    current.clear();
}

fn exact_identifier_boost(query: &str, doc: &[String]) -> f32 {
    tokenize(query)
        .into_iter()
        .filter(|token| is_identifier_like(token) && doc.iter().any(|doc_token| doc_token == token))
        .count() as f32
        * 1.5
}

fn is_identifier_like(token: &str) -> bool {
    token.contains('_')
        || token.contains('-')
        || token.contains('.')
        || token.contains('@')
        || token.chars().any(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_identifier_match_scores_above_generic_terms() {
        let corpus = Bm25Corpus::new([
            "general retry handling around worker state",
            "panic in sync_worker_v2 for account id 42",
            "worker completed normally after queue drain",
        ]);

        let scores = corpus.score_all("sync_worker_v2");

        assert!(scores[1].score > scores[0].score, "{scores:?}");
        assert!(scores[1].score > scores[2].score, "{scores:?}");
    }

    #[test]
    fn tokenization_keeps_dotted_and_email_identifiers() {
        let tokens = tokenize("api.worker.example failed for ops@example.com");

        assert!(tokens.contains(&"api.worker.example".to_string()));
        assert!(tokens.contains(&"ops@example.com".to_string()));
    }
}
