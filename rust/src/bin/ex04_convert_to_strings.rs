/*
Exercise 4: Convert to Strings

Task: Convert a vector of integers to a vector of strings representing each number.
Instructions:
- Create a vector with integers.
- Use map to convert each integer to a string.
- Collect the results into a new vector and print it.
*/
fn ex04() {
    let numbers: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let nums_converted: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();

    println!("Original  : {:?}", numbers);
    println!("Converted : {:?}", nums_converted);
}

fn main() {
    ex04();
}
