use async_std::task;
use std::time::Duration;

async fn async_print1(i: i32) {
    println!("number {} from the async_1", i);
    task::sleep(Duration::from_millis(500)).await;
}

async fn async_print2(i: i32) {
    println!("number {} from the async_2", i);
    task::sleep(Duration::from_millis(500)).await;
}

fn main() {
    let async1 = task::spawn(async {
        for i in 1..=5 {
            async_print1(i).await;
        }
    });
    let async2 = task::spawn(async {
        for i in 1..=5 {
            async_print2(i).await;
        }
    });

    task::block_on(async1);
    task::block_on(async2);
}
