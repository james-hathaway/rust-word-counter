
use std::collections::HashMap;

use regex::Regex;


fn main() {
    // constant variable to hold the path to the input file
    const file_path: &str = "";
    // Read the contents of the file into a string variable
    let text = std::fs::read_to_string(file_path).expect("Failed to read file");
    // here I am matching any alpha-numeric letters
    let re = Regex::new(r"[^\w\s]+").unwrap();
    // Replace any matches of the regular expression with an empty string
    let text = re.replace_all(&text, "");
    // Split the cleaned-up text into individual words
    let words = text.split_whitespace();
    // Create a new HashMap to store the word counts
    let mut word_counts = HashMap::new();
    // Iterate over each word in the cleaned-up text
    for word in words {
        // Insert the word into the HashMap with a count of 0 if it doesn't already exist,
        // and retrieve a mutable reference to the count for the word
        let count = word_counts.entry(word.to_string()).or_insert(0);
        // Increment the count for the word
        *count += 1;
    }
    // Iterate over each entry in the HashMap and print the word and its count
    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
}
