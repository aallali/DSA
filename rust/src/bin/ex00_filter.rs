/*
Exercise 0: Filter and Map
Task: Given a vector of integers, filter out the even numbers, square them, and collect the results into a new vector.

Instructions:

Create a vector with integers from 1 to 10.
Use the filter method to keep only even numbers.
Use the map method to square each of the even numbers.
Collect the results into a new vector and print it.
*/
fn ex00() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    numbers = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    println!("{:?}", numbers)
}

fn main() {
    ex00();
}
