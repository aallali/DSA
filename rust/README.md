Here's a Rust cheat sheet covering essential syntax and concepts to help you get started with LeetCode problems:

### Basic Syntax

#### Variables and Data Types

```rust
// Variables
let x = 5; // Immutable
let mut y = 10; // Mutable
y = y + 5;

// Data Types
let integer: i32 = 10;
let float: f64 = 10.5;
let boolean: bool = true;
let character: char = 'a';
let string: &str = "hello";
```

#### Functions

```rust
// Function Definition
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Calling a Function
let result = add(5, 10);
```

#### Control Flow

```rust
// If-Else
if x > 5 {
    println!("x is greater than 5");
} else {
    println!("x is not greater than 5");
}

// Loop
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break;
    }
}

// For Loop
for i in 0..5 {
    println!("{}", i);
}

// While Loop
while count < 5 {
    count += 1;
}
```

### Data Structures

#### Arrays and Vectors

```rust
// Array (fixed size)
let arr: [i32; 3] = [1, 2, 3];

// Vector (dynamic size)
let mut vec: Vec<i32> = vec![1, 2, 3];
vec.push(4);
vec.pop();
```

#### Tuples

```rust
// Tuple
let tup: (i32, f64, char) = (500, 6.4, 'a');
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```

### Structs and Enums

#### Structs

```rust
// Struct Definition
struct Person {
    name: String,
    age: u32,
}

// Creating an Instance
let person = Person {
    name: String::from("Alice"),
    age: 30,
};
```

#### Enums

```rust
// Enum Definition
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Using Enum
let move_dir = Direction::Up;
match move_dir {
    Direction::Up => println!("Moving Up"),
    Direction::Down => println!("Moving Down"),
    _ => println!("Other direction"),
}
```

### Error Handling

#### Option and Result

```rust
// Option
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

// Using Option
match divide(10, 2) {
    Some(result) => println!("Result: {}", result),
    None => println!("Cannot divide by zero"),
}

// Result
fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

// Using Result
match read_file("file.txt") {
    Ok(content) => println!("File content: {}", content),
    Err(error) => println!("Error reading file: {}", error),
}
```

### Ownership and Borrowing

```rust
// Ownership
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership is moved
    // println!("{}", s1); // This line would cause an error

    let s3 = s2.clone(); // Cloning the value
    println!("{}", s3);
}

// Borrowing
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrowing
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Error Messages and Debugging

Use `println!` for debugging:

```rust
println!("Debug: {:?}", variable);
```

### Common Collections

#### HashMap

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key1", 10);
map.insert("key2", 20);

if let Some(value) = map.get("key1") {
    println!("Value: {}", value);
}
```

In addition to `Vec` and `HashMap`, Rust provides several other common collections and utilities. Here’s a quick overview:

### `HashSet`

A `HashSet` is a collection of unique values, similar to `HashMap` but without associated values.

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
set.insert("apple"); // Duplicates are ignored

if set.contains("banana") {
    println!("Set contains banana");
}
```

### `BTreeMap`

A `BTreeMap` is a sorted map where keys are ordered.

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("b", 2);
map.insert("a", 1);
map.insert("c", 3);

for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

### `BTreeSet`

A `BTreeSet` is a sorted set where elements are ordered.

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(1);
set.insert(3);
set.insert(2);

for value in &set {
    println!("{}", value);
}
```

### `LinkedList`

A `LinkedList` provides a doubly-linked list with efficient insertions and deletions.

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(1);
list.push_front(2);

if let Some(value) = list.pop_front() {
    println!("Popped: {}", value);
}
```

### `VecDeque`

A `VecDeque` is a double-ended queue that allows efficient insertions and deletions from both ends.

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(2);

if let Some(value) = deque.pop_back() {
    println!("Popped from back: {}", value);
}
if let Some(value) = deque.pop_front() {
    println!("Popped from front: {}", value);
}
```

### `BinaryHeap`

A `BinaryHeap` is a priority queue where elements are ordered based on a heap property.

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(1);
heap.push(3);
heap.push(2);

while let Some(value) = heap.pop() {
    println!("{}", value);
}
```

These collections offer a variety of ways to manage and manipulate data efficiently in Rust. Each has its use cases, depending on whether you need ordered elements, unique values, or specific performance characteristics.

## Array/Vectors methods
Rust offers several iterator methods similar to JavaScript's array methods. Here’s a list of some common ones:

### `map`

Transforms each element of an iterator.

```rust
let numbers = vec![1, 2, 3];
let squares: Vec<i32> = numbers.into_iter().map(|x| x * x).collect();
println!("{:?}", squares); // [1, 4, 9]
```

### `filter`

Filters elements based on a predicate.

```rust
let numbers = vec![1, 2, 3, 4];
let evens: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
println!("{:?}", evens); // [2, 4]
```

### `find`

Finds the first element matching a predicate.

```rust
let numbers = vec![1, 2, 3, 4];
let first_even = numbers.into_iter().find(|&x| x % 2 == 0);
println!("{:?}", first_even); // Some(2)
```

### `any`

Checks if any elements match a predicate.
It stops after the first occurence of the condition

```rust
let numbers = vec![1, 2, 3, 4];
let has_even = numbers.into_iter().any(|x| x % 2 == 0);
println!("{}", has_even); // true
```

### `all`

Checks if all elements match a predicate.

```rust
let numbers = vec![2, 4, 6];
let all_even = numbers.into_iter().all(|x| x % 2 == 0);
println!("{}", all_even); // true
```

### `for_each`

Executes a closure on each element.

```rust
let numbers = vec![1, 2, 3];
numbers.into_iter().for_each(|x| println!("{}", x));
// Outputs: 1 2 3
```

### `fold`

Accumulates a value by applying a closure.

```rust
let numbers = vec![1, 2, 3];
let sum: i32 = numbers.into_iter().fold(0, |acc, x| acc + x);
println!("{}", sum); // 6
```

### `reduce`

Reduces the elements of an iterator to a single value, similar to `fold` but without an initial value.

```rust
let numbers = vec![1, 2, 3];
let sum = numbers.into_iter().reduce(|acc, x| acc + x);
println!("{:?}", sum); // Some(6)
```

### `collect`

Transforms an iterator into a collection, like a `Vec` or `HashSet`.

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
println!("{:?}", doubled); // [2, 4, 6]
```
