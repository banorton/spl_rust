// Ownership and borrowing examples

fn main() {
    // Move semantics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // Error: s1 no longer valid
    println!("{}", s2);

    // Borrowing (immutable)
    let s3 = String::from("world");
    let len = calculate_length(&s3);
    println!("Length of '{}' is {}", s3, len);

    // Mutable borrowing
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("{}", s4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
