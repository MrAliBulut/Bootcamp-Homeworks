# Bootcamp Homework 2

## Table of Contents

- [Task Details](#task-details)
- [Steps](#steps)
- [Usage](#usage)
- [License](#license)

## Task Details

In this homework, I am to create a simple Rust program that represents a simple Library Management project with just Book and Magazine objects. 

## Steps

1. Define an enum named `Publication` with two variants: `Book` and `Magazine`. Each variant should contain different data types:
   - The `Book` variant should contain a struct with fields `title`, `author`, and `page_count`.
   - The `Magazine` variant should contain a struct with fields `title`, `issue`, and `topic`.
2. Implement a function called `print_publication` that takes a `Publication` enum as input and prints the publication information according to its type (book or magazine). For example, use the format "Book: [title] Author: [author], Page Count: [page_count]" for books and "Magazine: [title] - Issue: [issue], Topic: [topic]" for magazines.
3. Create instances of books and magazines, and store them in a `Vec<Publication>` array.
4. Call the `print_publication` function to print each item.
5. Compile and run the program to verify its correctness.

## Usage

Use this command after downloading the code:

```bash
cargo run --bin bootcamp_homework2
```

This will compile and execute the `bootcamp_homework2` binary.

## License

This code is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
