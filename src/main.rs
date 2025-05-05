use clap::{Parser, ValueEnum};
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "case-converter")]
#[command(about = "Converts text between different case styles")]
struct Args {
    /// The case to convert to
    #[arg(short = 'c', long = "case", value_enum)]
    case: Case,

    /// The input text to convert (if not provided, reads from stdin)
    text: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Case {
    #[value(name = "snake_case")]
    #[value(alias = "snake")]
    #[value(help = "snake_case format (alias: snake)")]
    Snake,

    #[value(name = "UPPER_SNAKE_CASE")]
    #[value(alias = "upper_snake")]
    #[value(alias = "upper-snake")]
    #[value(help = "UPPER_SNAKE_CASE format (aliases: upper_snake, upper-snake)")]
    UpperSnake,

    #[value(name = "camelCase")]
    #[value(alias = "camel")]
    #[value(help = "camelCase format (alias: camel)")]
    Camel,

    #[value(name = "PascalCase")]
    #[value(alias = "pascal")]
    #[value(help = "PascalCase format (alias: pascal)")]
    Pascal,

    #[value(name = "kebab-case")]
    #[value(alias = "kebab")]
    #[value(help = "kebab-case format (alias: kebab)")]
    Kebab,
}

fn main() {
    let args = Args::parse();
    
    // Get the input text either from command line arguments or stdin
    let input = match args.text {
        Some(text) => text,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
            buffer.trim().to_string()
        }
    };
    
    // Convert the input to the desired case
    let output = convert_case(&input, args.case);
    
    // Output the result
    println!("{}", output);
}

fn convert_case(input: &str, case: Case) -> String {
    // First, we normalize the input by splitting it into words
    let words = split_into_words(input);
    
    // Then convert to the desired case
    match case {
        Case::Snake => words.join("_").to_lowercase(),
        Case::UpperSnake => words.join("_").to_uppercase(),
        Case::Camel => {
            if words.is_empty() {
                String::new()
            } else {
                let mut result = words[0].to_lowercase();
                for word in &words[1..] {
                    result.push_str(&capitalize(word));
                }
                result
            }
        },
        Case::Pascal => {
            words.iter()
                .map(|word| capitalize(word))
                .collect::<Vec<_>>()
                .join("")
        },
        Case::Kebab => words.join("-").to_lowercase(),
    }
}

fn split_into_words(input: &str) -> Vec<String> {
    // Handle empty input
    if input.is_empty() {
        return vec![];
    }

    // This is a simple heuristic to split the input into words
    // It handles camelCase, PascalCase, snake_case, kebab-case
    let mut words = Vec::new();
    let mut current_word = String::new();
    
    // Helper to add the current word to our words list and reset it
    let add_word = |words: &mut Vec<String>, current_word: &mut String| {
        if !current_word.is_empty() {
            words.push(current_word.clone());
            current_word.clear();
        }
    };
    
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            // Skip separators, but add the current word if we have one
            '_' | '-' | ' ' => {
                add_word(&mut words, &mut current_word);
            },
            // For uppercase letters, we might need to start a new word
            c if c.is_uppercase() => {
                // If we're not at the start of the input and the current word is not empty,
                // and the previous character is lowercase, we start a new word
                if i > 0 && !current_word.is_empty() && chars[i-1].is_lowercase() {
                    add_word(&mut words, &mut current_word);
                }
                current_word.push(c);
            },
            // For any other character, just add it to the current word
            c => {
                current_word.push(c);
            }
        }
        i += 1;
    }
    
    // Don't forget to add the last word
    add_word(&mut words, &mut current_word);
    
    words
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}