// Async/await examples (requires tokio)
// Add to Cargo.toml: tokio = { version = "1", features = ["full"] }

use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting async examples");

    // Simple async function
    let result = fetch_data().await;
    println!("Fetched: {}", result);

    // Concurrent execution with join
    let (a, b, c) = tokio::join!(
        fetch_data(),
        process_data("hello"),
        process_data("world")
    );
    println!("Results: {}, {}, {}", a, b, c);

    // Spawn tasks
    let handle1 = tokio::spawn(async {
        fetch_data().await
    });
    let handle2 = tokio::spawn(async {
        process_data("async").await
    });

    let res1 = handle1.await.unwrap();
    let res2 = handle2.await.unwrap();
    println!("Spawned results: {}, {}", res1, res2);
}

async fn fetch_data() -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "data".to_string()
}

async fn process_data(input: &str) -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    format!("processed: {}", input)
}
