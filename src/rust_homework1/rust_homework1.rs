fn concatenate_strings(string1:&str, string2:&str) -> String  {

 let mut result = String::new();
 result.push_str(string1);
 result.push_str(string2);
 return result;

}
fn main() {
    let string1 = String::from("Hello,");
    let string2 = String::from(" Rust!");
    let concatenated_strings = concatenate_strings(&string1, &string2);

    println!("{}",concatenated_strings);

}

