/*
Exercise 10: Peekable Iterators

Task: Use a peekable iterator to inspect elements before consuming them.

Instructions:
- Create a vector of integers.
- Convert it to a peekable iterator.
- Peek at the next element, then consume it.
- Print the element you peeked at and the remaining elements.
*/
fn ex10() {
    let numbers: Vec<i32> = vec![1, 3, 3, 7];

    let mut iter = numbers.iter().peekable();

    let peek1 = iter.peek().copied();
    let peek1_twice = iter.peek().copied();
    let consume1 = iter.next().copied();
    let peek2 = iter.peek().copied();
    let remaining_numbers = iter.collect::<Vec<&i32>>();
    println!("Peek Next element {:?}", peek1);
    println!(
        "Peek Next element {:?} // stay the same even peek multiple time",
        peek1_twice
    );
    println!("Consume Next element {:?}", consume1);
    println!("Peek next element {:?}", peek2);
    println!(
        "Print Remanining elements after peek {:?}",
        remaining_numbers
    );
}

fn main() {
    ex10();
}
