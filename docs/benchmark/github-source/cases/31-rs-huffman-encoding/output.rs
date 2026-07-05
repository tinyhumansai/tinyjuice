//! Huffman Encoding implementation
//!
//! Huffman coding is a lossless data compression algorithm that assigns variable-length codes
//! to characters based on their frequency of occurrence. Characters that occur more frequently
//! are assigned shorter codes, while less frequent characters get longer codes.
//!
//! # Algorithm Overview
//!
//! 1. Count the frequency of each character in the input
//! 2. Build a min-heap (priority queue) of nodes based on frequency
//! 3. Build the Huffman tree by repeatedly:
//!    - Remove two nodes with minimum frequency
//!    - Create a parent node with combined frequency
//!    - Insert the parent back into the heap
//! 4. Traverse the tree to assign binary codes to each character
//! 5. Encode the input using the generated codes
//!
//! # Time Complexity
//!
//! - Building frequency map: O(n) where n is input length
//! - Building Huffman tree: O(m log m) where m is number of unique characters
//! - Encoding: O(n)
//!
//! # Usage
//!
//! As a library:
//! ```no_run
//! use the_algorithms_rust::compression::huffman_encode;
//!
//! let text = "hello world";
//! let (encoded, codes) = huffman_encode(text);
//! println!("Original: {}", text);
//! println!("Encoded: {}", encoded);
//! ```
//!
//! As a command-line tool:
//! ```bash
//! rustc huffman_encoding.rs -o huffman
//! ./huffman input.txt
//! ```

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[cfg(not(test))]
use std::env;

/// Represents a node in the Huffman tree
#[derive(Debug, Eq, PartialEq)]
enum HuffmanNode {
    /// Leaf node containing a character and its frequency
    Leaf { character: char, frequency: usize },
    /// Internal node with combined frequency and left/right children
    Internal {
        frequency: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

impl HuffmanNode {
    /// Returns the frequency of this node
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Leaf { frequency, .. } | HuffmanNode::Internal { frequency, .. } => {
                *frequency
            }
        }
    }

    /// Creates a new leaf node
    fn new_leaf(character: char, frequency: usize) -> Self {
        HuffmanNode::Leaf {
            character,
            frequency,
        }
    }

    /// Creates a new internal node from two children
    fn new_internal(left: HuffmanNode, right: HuffmanNode) -> Self {
        let frequency = left.frequency() + right.frequency();
        HuffmanNode::Internal {
            frequency,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Wrapper for HuffmanNode to implement Ord for BinaryHeap (min-heap)
#[derive(Eq, PartialEq)]
struct HeapNode(HuffmanNode);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.0.frequency().cmp(&self.0.frequency())
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Counts the frequency of each character in the input string
///
/// # Arguments
///
/// * `text` - The input string to analyze
///
/// # Returns
///
/// A HashMap mapping each character to its frequency count
fn build_frequency_map(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    for ch in text.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }
    frequencies
}

/// Builds the Huffman tree from a frequency map
///
/// # Arguments
///
/// * `frequencies` - HashMap of character frequencies
///
/// # Returns
///
/// The root node of the Huffman tree, or None if input is empty
fn build_huffman_tree(frequencies: HashMap<char, usize>) -> Option<HuffmanNode> {
    if frequencies.is_empty() {
        return None;
    { … 20 line(s) … ⟦tj:064d775e95f8871ec78f7fd58d176e86⟧ }
    heap.pop().map(|node| node.0)
}

/// Traverses the Huffman tree to generate binary codes for each character
///
/// # Arguments
///
/// * `node` - The current node being traversed
/// * `code` - The current binary code string
/// * `codes` - HashMap to store the generated codes
fn generate_codes(node: &HuffmanNode, code: String, codes: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { character, .. } => {
    { … 14 line(s) … ⟦tj:b2545f120ee7414ce39ae09952c97947⟧ }
    }
}

/// Encodes text using Huffman coding
///
/// # Arguments
///
/// * `text` - The input string to encode
///
/// # Returns
///
/// A tuple containing:
/// - The encoded binary string
/// - A HashMap of character to binary code mappings
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use the_algorithms_rust::compression::huffman_encode;
/// let (encoded, codes) = huffman_encode("hello");
/// assert!(!encoded.is_empty());
/// assert!(codes.contains_key(&'h'));
/// ```
pub fn huffman_encode(text: &str) -> (String, HashMap<char, String>) {
    if text.is_empty() {
        return (String::new(), HashMap::new());
    { … 10 line(s) … ⟦tj:c652b7d04a32ef25b2ab55585f8d6d18⟧ }
    (encoded, codes)
}

/// Decodes a Huffman-encoded string
///
/// # Arguments
///
/// * `encoded` - The binary string to decode
/// * `codes` - HashMap of character to binary code mappings
///
/// # Returns
///
/// The decoded original string
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use the_algorithms_rust::compression::{huffman_encode, huffman_decode};
/// let text = "hello world";
/// let (encoded, codes) = huffman_encode(text);
/// let decoded = huffman_decode(&encoded, &codes);
/// assert_eq!(text, decoded);
/// ```
pub fn huffman_decode(encoded: &str, codes: &HashMap<char, String>) -> String {
    if encoded.is_empty() {
        return String::new();
    { … 19 line(s) … ⟦tj:814c2f954559ff95c1e75e4dc73820c9⟧ }
    decoded
}

/// Demonstrates Huffman encoding by processing a file and displaying detailed results
///
/// This function reads a file, encodes it using Huffman coding, and displays:
/// - Character code mappings
/// - Compression statistics
/// - Encoded output (with smart truncation for large files)
/// - Decoding verification
///
/// # Arguments
///
/// * `file_path` - Path to the file to encode
///
/// # Returns
///
/// Result indicating success or IO error
///
/// # Examples
///
/// ```ignore
/// // Note: This function is not re-exported in the public API
/// // Access it via: the_algorithms_rust::compression::huffman_encoding::demonstrate_huffman_from_file
/// use std::fs::File;
/// use std::io::Write;
///
/// // Create a test file
/// let mut file = File::create("test.txt").unwrap();
/// file.write_all(b"hello world").unwrap();
///
/// // Demonstrate Huffman encoding
/// // In your code, use the full path or import from huffman_encoding module
/// demonstrate_huffman_from_file("test.txt").unwrap();
/// ```
#[allow(dead_code)]
pub fn demonstrate_huffman_from_file(file_path: &str) -> std::io::Result<()> {
    // Read the file contents
    let text = fs::read_to_string(file_path)?;
    { … 84 line(s) … ⟦tj:7d015b08e96d0e5f84e4d4cc8aef60f7⟧ }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let (encoded, codes) = huffman_encode("");
        assert_eq!(encoded, "");
        assert!(codes.is_empty());
    }

    #[test]
    fn test_single_character() {
        let (encoded, codes) = huffman_encode("aaaa");
        assert_eq!(encoded, "0000");
        assert_eq!(codes.get(&'a'), Some(&"0".to_string()));
    }

    #[test]
    fn test_simple_string() { … 13 line(s) … ⟦tj:b5d3168afcefd227097185cf4a892d56⟧ }

    #[test]
    fn test_encode_decode_roundtrip() {
        let test_cases = vec![
            "a",
        { … 10 line(s) … ⟦tj:458b4802df972876258794181648e230⟧ }
        }
}

    #[test]
    fn test_frequency_based_encoding() { … 11 line(s) … ⟦tj:c04a515a030b920e368aa1913b936520⟧ }

    #[test]
    fn test_compression_ratio() {
        let text = "aaaaaaaaaa"; // 10 'a's
        let (encoded, _) = huffman_encode(text);

        // Original: 10 chars * 8 bits = 80 bits (in UTF-8)
        // Huffman: 10 * 1 bit = 10 bits (single character gets code "0")
        assert_eq!(encoded.len(), 10);
        assert!(encoded.chars().all(|c| c == '0'));
    }

    #[test]
    fn test_all_unique_characters() { … 11 line(s) … ⟦tj:1e5e6ba854ac5f255e7b42126d713fb8⟧ }

    #[test]
    fn test_build_frequency_map() {
        let frequencies = build_frequency_map("hello");
        assert_eq!(frequencies.get(&'h'), Some(&1));
        assert_eq!(frequencies.get(&'e'), Some(&1));
        assert_eq!(frequencies.get(&'l'), Some(&2));
        assert_eq!(frequencies.get(&'o'), Some(&1));
    }

    #[test]
    fn test_unicode_characters() {
        let text = "Hello, 世界! 🌍";
        let (encoded, codes) = huffman_encode(text);
        let decoded = huffman_decode(&encoded, &codes);
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_demonstrate_huffman_from_file() {
        use std::fs::File;
        use std::io::Write;
        { … 12 line(s) … ⟦tj:fd25005adf28e082faa828e72fdc7e69⟧ }
        assert!(result.is_ok());
}

    #[test]
    fn test_demonstrate_empty_file() { … 11 line(s) … ⟦tj:1df06dbde8733d172cd00ee10b4737e0⟧ }
}

/// Main function for command-line usage
///
/// Allows this file to be compiled as a standalone binary:
/// ```bash
/// rustc huffman_encoding.rs -o huffman
/// ./huffman input.txt
/// ```
#[cfg(not(test))]
#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Huffman Encoding - Lossless Data Compression");
        eprintln!();
        eprintln!("Usage: {} <file_path>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} sample.txt", args[0]);
        eprintln!();
        eprintln!("This will encode the file and display:");
        eprintln!("  - Character code mappings");
        eprintln!("  - Compression statistics");
        eprintln!("  - Encoded binary output");
        eprintln!("  - Verification of successful decoding");
        std::process::exit(1);
    }

    let file_path = &args[1];

    match demonstrate_huffman_from_file(file_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error processing file '{file_path}': {e}");
            std::process::exit(1);
        }
    }
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (16293 bytes): call tinyjuice_retrieve with token "e84401ece504123913332f21543191c2"]