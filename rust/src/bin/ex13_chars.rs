fn ex13() -> bool {
    let s: String = String::from("abcdea");
    let t: String = String::from("abcdea");

    let mut letter: [i32; 26] = [0; 26];

    for c in s.chars() {
        letter[c as usize - 97] += 1;
    }

    for c in t.chars() {
        let idx = c as usize - 97;
        if letter[idx] == 0 {
            return false;
        }
        letter[idx] -= 1;
    }
 

    return true;
}

fn main() {
    println!("{}", ex13());
}
