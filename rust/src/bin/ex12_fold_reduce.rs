/*
Exercise 12: Fold and Reduce

Task: Use fold to compute the product of all integers in a vector.

Instructions:
- Create a vector of integers.
- Use fold to compute the product of all elements.
- Print the result.
*/
fn ex12() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("nums: {:?}\n", numbers);

    let prod = numbers.iter().fold(1, |acc, x| acc * x);
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Using `fold()` method: ");
    println!("prod: {:?}", prod);
    println!("sum: {:?}\n", sum);

    let prod = numbers.clone().into_iter().reduce(|acc, x| x * acc).unwrap();
    let sum = numbers.into_iter().reduce(|acc, x| acc + x).unwrap();
    println!("Using `reduce()` method: ");
    println!("prod: {:?}", prod);
    println!("sum: {:?}", sum);
}

fn main() {
    ex12();
}
