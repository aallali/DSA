/*
Exercise 6: Reverse Position

Task: Given a vector of integers, find the position of the last occurrence of an integer that is greater than 5.

Instructions:
- Create a vector with integers.
- Use rposition to find the index of the last element greater than 5.
- Print the position.
 */
fn ex06() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let index = numbers.iter().rposition(|&n| n > 5);

    match index {
        Some(idx) => println!("numbers[{:?}] = {}", idx, numbers.get(idx).unwrap()),
        None => println!("NOT FOUND"),
    }
}

fn main() {
    ex06();
}
