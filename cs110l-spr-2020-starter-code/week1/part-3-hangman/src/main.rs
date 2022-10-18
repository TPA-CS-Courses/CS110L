// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut plant_word = String::from("--------"); 
    let mut plant_word_chars: Vec<char> = plant_word.chars().collect();
    let mut guessed_word = String::new();
    let mut num_left = secret_word.len();
    loop {
        println!("The word so far is {}", plant_word);
        println!("You have guessed the following letters: {}", guessed_word);
        println!("You have {} guesses left", num_left);
        
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line.");
        let char_vec: Vec<char> = guess.chars().collect();
        let char_read = char_vec[0];
        guessed_word.push(char_read);
        // println!("read: {}", );
        let mut flag = false;
        for i in 0 .. secret_word_chars.len() {
            println!("plant:{}, secret:{}", plant_word_chars[i], secret_word_chars[i]);
            if plant_word_chars[i] == '-' && secret_word_chars[i] == char_read {
                plant_word_chars[i] = char_read;
                num_left = num_left - 1;
                flag = true;
                break;
            } 
        }
        if flag == false {
            println!("Sorry, that letter is not in the word");
        }
        println!("");
        println!("");
    }
}
