/*
Filtering and Aggregation
Exercise 8: Take and Skip

Task: Given a vector of integers, use take to get the first three elements and skip to get the remaining elements after the first three.

Instructions:
- Create a vector with integers from 1 to 10.
- Use take to get the first three elements.
- Use skip to get the remaining elements after the first three.
- Print both results.
*/

fn ex08() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use `take` to get the first three elements
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("First three elements: {:?}", first_three); // [1, 2, 3]

    // Use `skip` to get the remaining elements after the first three
    let remaining_elements: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("Remaining elements after the first three: {:?}", remaining_elements); // [4, 5, 6, 7, 8, 9, 10]

    numbers.push(1337);
    println!("Original Vec after update: {:?}", numbers); // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1337]
    // println!("Remaining elements after the first three: {:?}", remaining_elements); // error[E0502]: cannot borrow `numbers` as mutable because it is also borrowed as immutable
}

fn main() {
    ex08();
}
