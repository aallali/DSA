/*

Advanced Iterators
Exercise 5: Find and Inspect

Task: Given a vector of strings, find the first string that contains the letter 'a', and use inspect to print each string during the iteration.

Instructions:
- Create a vector with strings: ["apple", "banana", "cherry"].
- Use inspect to print each string.
- Use find to get the first string containing 'a' and print it.
*/

fn ex05() {
    let fruits: Vec<&str> = vec!["apple", "bananna", "cherry"];
    let _: Vec<_> = fruits
        .iter()
        .inspect(|&&f| println!("Fruit : {:?}", f))
        .collect();
    let first_match = fruits
        .iter()
        .find(|&&f| f.contains('b'))
        .unwrap_or_else(|| &"NOT_FOUND");
    println!("First Match: {}", first_match)
}

fn main() {
    ex05();
}
