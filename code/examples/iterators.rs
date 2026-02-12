// Iterator examples

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // map and collect
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // filter
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // fold (reduce)
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    // chain
    let more_numbers = vec![6, 7, 8];
    let combined: Vec<i32> = numbers.iter()
        .chain(more_numbers.iter())
        .copied()
        .collect();
    println!("Combined: {:?}", combined);

    // Custom iterator
    let counter = Counter::new();
    let result: Vec<i32> = counter.take(5).collect();
    println!("Counter: {:?}", result);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 10 {
            Some(self.count as i32)
        } else {
            None
        }
    }
}
