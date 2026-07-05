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
    fn frequency(&self) -> usize { … 7 line(s) … ⟦tj:fcaca3412281313ef2e2bc2269be17cc⟧ }

    /// Creates a new leaf node
    fn new_leaf(character: char, frequency: usize) -> Self { … 6 line(s) … ⟦tj:81f1041a2cff495304a91c4ee2508579⟧ }

    /// Creates a new internal node from two children
    fn new_internal(left: HuffmanNode, right: HuffmanNode) -> Self { … 8 line(s) … ⟦tj:6fb6cc38a54c0125b0a0cb5c13fe9b8a⟧ }
}

/// Wrapper for HuffmanNode to implement Ord for BinaryHeap (min-heap)
#[derive(Eq, PartialEq)]
struct HeapNode(HuffmanNode);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering { … 4 line(s) … ⟦tj:54c4bf382ca056036cce508154963f76⟧ }
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
fn build_frequency_map(text: &str) -> HashMap<char, usize> { … 7 line(s) … ⟦tj:db754fc7a73b5e4810d9e9b4516766a2⟧ }

/// Builds the Huffman tree from a frequency map
///
/// # Arguments
///
/// * `frequencies` - HashMap of character frequencies
///
/// # Returns
///
/// The root node of the Huffman tree, or None if input is empty
fn build_huffman_tree(frequencies: HashMap<char, usize>) -> Option<HuffmanNode> { … 25 line(s) … ⟦tj:064d775e95f8871ec78f7fd58d176e86⟧ }

/// Traverses the Huffman tree to generate binary codes for each character
///
/// # Arguments
///
/// * `node` - The current node being traversed
/// * `code` - The current binary code string
/// * `codes` - HashMap to store the generated codes
fn generate_codes(node: &HuffmanNode, code: String, codes: &mut HashMap<char, String>) { … 19 line(s) … ⟦tj:b2545f120ee7414ce39ae09952c97947⟧ }

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
pub fn huffman_encode(text: &str) -> (String, HashMap<char, String>) { … 15 line(s) … ⟦tj:c652b7d04a32ef25b2ab55585f8d6d18⟧ }

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
pub fn huffman_decode(encoded: &str, codes: &HashMap<char, String>) -> String { … 24 line(s) … ⟦tj:814c2f954559ff95c1e75e4dc73820c9⟧ }

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
pub fn demonstrate_huffman_from_file(file_path: &str) -> std::io::Result<()> { … 89 line(s) … ⟦tj:7d015b08e96d0e5f84e4d4cc8aef60f7⟧ }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() { … 5 line(s) … ⟦tj:4e778dccd16d1f7862a92cf401c0f195⟧ }

    #[test]
    fn test_single_character() { … 5 line(s) … ⟦tj:aa6b982f5efa0b2a951505bf46c42815⟧ }

    #[test]
    fn test_simple_string() { … 13 line(s) … ⟦tj:b5d3168afcefd227097185cf4a892d56⟧ }

    #[test]
    fn test_encode_decode_roundtrip() { … 15 line(s) … ⟦tj:458b4802df972876258794181648e230⟧ }

    #[test]
    fn test_frequency_based_encoding() { … 11 line(s) … ⟦tj:c04a515a030b920e368aa1913b936520⟧ }

    #[test]
    fn test_compression_ratio() { … 9 line(s) … ⟦tj:25618276f39d4ddf267eb526fa998b66⟧ }

    #[test]
    fn test_all_unique_characters() { … 11 line(s) … ⟦tj:1e5e6ba854ac5f255e7b42126d713fb8⟧ }

    #[test]
    fn test_build_frequency_map() { … 7 line(s) … ⟦tj:4a07e035aa8a4cc30bb538dd2933016f⟧ }

    #[test]
    fn test_unicode_characters() { … 6 line(s) … ⟦tj:fe67215d186967a29297df4d484b3007⟧ }

    #[test]
    fn test_demonstrate_huffman_from_file() { … 17 line(s) … ⟦tj:fd25005adf28e082faa828e72fdc7e69⟧ }

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
fn main() { … 29 line(s) … ⟦tj:48e3d59882f81dea3c39acb40ee31029⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (16293 bytes) is available by calling tinyjuice_retrieve with token "e84401ece504123913332f21543191c2" (marker ⟦tj:e84401ece504123913332f21543191c2⟧)]