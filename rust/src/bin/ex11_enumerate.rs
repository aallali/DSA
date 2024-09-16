/*
Exercise 11: Enumerate

Task: Use enumerate to get each element along with its index.

Instructions:
- Create a vector of strings.
- Use enumerate to pair each element with its index.
- Print the index and the corresponding element.
*/

fn ex11() {
    let charos = vec!['a', 'b', 'c', 'd', 'c', 'd', 'e'];
    println!(
        "indexes: {:?}",
        charos
            .iter()
            .enumerate()
            .map(|(i, _c)| i)
            .collect::<Vec<usize>>()
    );
    println!(
        "chars: {:?}",
        charos
            .iter()
            .enumerate()
            .map(|(_i, c)| c)
            .collect::<Vec<&char>>()
    );

    for (i, c) in charos.iter().enumerate() {
        println!("{} - {}", i, c)
    }
}

fn main() {
    ex11();
}
