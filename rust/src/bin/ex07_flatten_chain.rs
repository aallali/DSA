/*

Exercise 7: Flatten and Chain

Task: Given a vector of vectors of integers, flatten the nested vectors and then chain with another vector of integers.

Instructions:
- Create a vector of vectors: vec![vec![1, 2], vec![3, 4]].
- Use flat_map to flatten the nested vectors.
- Create another vector: vec![5, 6].
- Use chain to combine the flattened vector with the second vector.
- Collect and print the results.
*/

fn ex07() {
    let vecs: Vec<Vec<i8>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    println!("{:?}", vecs); // Output: [[1, 2], [3, 4], [5, 6]]

    let mut vecs: Vec<i8> = vecs.into_iter().flatten().collect();
    println!("{:?}", vecs); // Output: [1, 2, 3, 4, 5, 6]
    
    let vecum: Vec<i8> = vec![7, 8];
    // vecs.append(&mut vecum);
    // vecs.append(&mut vec![7, 8]);

    vecs = vecs.into_iter().chain(vecum.into_iter()).collect();
    println!("{:?}", vecs); // Output: [1, 2, 3, 4, 5, 6, 7, 8]
}

fn main() {
    ex07();
}
