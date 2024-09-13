/*
Exercise 3: Zip and Unzip
Task: Given two vectors, one with names and the other with ages, zip them into a vector of tuples. Then, unzip the vector of tuples back into two separate vectors.

Instructions:
- Create two vectors: one with names (["Alice", "Bob", "Carol"]) and one with ages ([30, 25, 35]).
- Use the zip method to combine them into a vector of tuples.
- Use the unzip method to separate the tuples back into two vectors.
- Print the original vectors, the zipped vector, and the unzipped vectors.
 */
fn ex02() {
    let names: Vec<&str> = vec!["Alice", "Bob", "Carol"];
    let ages: Vec<i32> = vec![30, 25, 35];
    let mut zip: Vec<(&str, i32)> = names
        .into_iter()
        .zip(
            // zip names to ages
            ages.into_iter(),
        ) // impl Iterator <Item= (&str, i32)>
        .collect();
    zip.push(("Abdellah", 1337));

    println!("{:#?}", zip);
    // Convert the Zip back into its original case
    let (n, a): (Vec<&str>, Vec<i32>) = zip.into_iter().unzip();

    println!("Names: {:#?}", n);
    println!("Ages: {:#?}", a);

}
fn main() {
    ex02();
}
