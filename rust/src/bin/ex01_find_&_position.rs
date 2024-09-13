/*
Exercise 1: Find and Position
Task: Given a vector of strings, find the first string that starts with the letter 'a',
and also find the position of the first string that contains the letter 'e'.

Instructions:
- Create a vector with strings: ["apple", "banana", "cherry", "date"].
- Use the find method to get the first string that starts with 'a'.
- Use the position method to find the index of the first string that contains the letter 'e'.
- Print both the found string and the position.
*/
fn ex01() {
    let words: Vec<&str> = vec!["apple", "banana", "cherry", "date"];

    let start_with_a: &&str = words
        .iter()
        .find(|&&w| w.starts_with('a'))
        .unwrap_or_else(|| &"NOT_FOUND");

    println!("First word starts with 'a' is: [{:?}]", start_with_a);
    let char_target = 't';
    let start_with_e_index = words.iter().position(|&w| w.contains(char_target));
    match start_with_e_index {
        Some(index) => println!(
            "Index of word contains with '{char_target}' is: [{:?}] | word is : [{}]",
            index, words.get(index).unwrap()
        ),
        None => println!("No word found containg that character {char_target}"),
    }
}
fn main() {
    ex01();
}
