// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use std::fmt::format;

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars(); //e Strings in rust aren't directly indexable because of utf-8. So we use .chars() to get an iterator over characters..
    match chars.next() { //e We take the first character, next() returns Option<char> (either Some(char) or None)
        None => String::new(),
        Some(first) => format!("{}{}",first.to_uppercase(),chars.as_str()) //e There should be no space between spaceholders
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    let result = words.iter().map(|x| capitalize_first(x)).collect();
    result 
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    let result = words.iter().map(|x| capitalize_first(x)).collect();
    result 
}

fn main() {
    // You can optionally experiment here.
    let small_string = "allen";
    let result = capitalize_first(small_string);
    println!("{}",result);

    let words = vec!["hello"," ", "world"];

    let capitalize_words_string_result = capitalize_words_string(&words);
    println!("{}",capitalize_words_string_result);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
