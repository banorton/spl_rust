// Pattern matching examples

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // Basic match
    let number = 7;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        13..=19 => println!("Teen"),
        _ => println!("Other"),
    }

    // Match with enums
    let msg = Message::ChangeColor(0, 160, 255);
    process_message(msg);

    // if let syntax
    let favorite_color: Option<&str> = Some("blue");
    if let Some(color) = favorite_color {
        println!("Favorite color: {}", color);
    }

    // while let syntax
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    }
}
