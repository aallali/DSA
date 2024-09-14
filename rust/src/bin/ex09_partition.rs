/*
4. Filtering and Aggregation
Exercice9 : Partition

Task: Partition a vector of integers into two vectors: one with even numbers and one with odd numbers.

Instructions:
- Create a vector with integers.
- Use partition to split the vector into even and odd numbers.
- Print both vectors.
*/

fn ex09() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter().partition(|&&x| x % 2 == 0);

    println!("Numbers: {:?}", numbers);
    // println!("{:?}", (evens, odds));
    println!("Evens  : {:?}", evens);
    println!("Odds   : {:?}", odds);
}

fn main() {
    ex09();
}
