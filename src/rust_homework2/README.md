# Rust Homework-1

## Table of Contents

- [Task Details](#task-details)
- [Steps](#steps)
- [Checklist](#checklist)
- [Usage](#usage)
- [License](#license)

## Task Details

In this homework, I am to create a simple Rust program that counts the number of words in a string taken as a input from the user. 

## Steps

1. Create a struct named `WordCounter`.
2. Inside the `WordCounter` struct, define a field named `text` to store the text to count words from.
3. Implement the following functions for `WordCounter`:
   - `new(text: &str) -> WordCounter`: This function should take a text and create an instance of the `WordCounter` struct.
   - `count_words() -> usize`: This function should count the number of words in the text and return the result as an integer. A word is defined as a string separated by whitespace characters.
4. In the `main` function, prompt the user to enter a text.
5. Read the input text entered by the user.
6. Create an instance of `WordCounter` using the entered text.
7. Call the `count_words` function on the created `WordCounter` instance.
8. Print the obtained word count to the screen.
9. Compile and run the program to test its functionality.

## Checklist

- [x] Define a struct named `WordCounter`.
- [x] In the `WordCounter` struct, define a field named `text`.
- [x] Implement the `new` function.
- [x] Implement the `count_words` function.
- [x] Prompt the user for text input.
- [x] Create a `WordCounter` instance using the input text.
- [x] Call the `count_words` function and print the word count to the screen.
- [x] Compile and run the program to test its functionality.

## Usage

Use this command after downloading the code:

```bash
cargo run --bin rust_homework2
```

This will compile and execute the `rust_homework2` binary.

## License

This code is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
