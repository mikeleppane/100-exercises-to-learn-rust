#![allow(dead_code)]
use std::sync::Arc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Arc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
