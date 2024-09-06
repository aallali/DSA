 

fn main() {
    let str: &str = "The Bombalu learning rust like ... ";

    for (i, c) in str.chars().rev().enumerate() {
        println!("str[{i}] = '{c}'")
    }
}
