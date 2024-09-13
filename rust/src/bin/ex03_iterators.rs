/*
Exercise 3: Sum of Squares

Task: Given a vector of integers, compute the sum of the squares of all even numbers.
Instructions:
- Create a vector with integers from 1 to 10.
- Use filter to keep only even numbers.
- Use map to square each number.
- Use sum to get the total sum and print it.
*/

fn ex03() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 10];
    println!("Original Vector: {:?}", vector);

    vector = vector.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Only even numbers: {:?}", vector);

    vector = vector.into_iter().map(|x| x * x).collect();
    println!("Power of each element: {:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("Sum of final vector: {:#?}", sum);
}

fn main() {
    ex03();
}
