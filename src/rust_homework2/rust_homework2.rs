struct WordCounter{
    text: String,
}

impl WordCounter{
    fn new(text: &str) -> WordCounter{
        let new_word_counter = WordCounter{text:String::from(text)};
        new_word_counter
    }

    fn count_words(&self) -> usize {

        let count: usize = self.text.split_whitespace().count();
        count
    }
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}
fn main(){

    println!("\n================================================================\n");

    println!("Please enter the text that you wish its word count:");
    let input_text = read_string();

    let counter = WordCounter::new(&input_text);
    let word_count= counter.count_words();

    println!("\nWord count in the text provided:\n{}", word_count);

    println!("\n================================================================\n");
}