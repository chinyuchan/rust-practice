use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(3);
    for i in 1..=5 {
        pool.execute(move || {
            println!("number {} from the  spawned_1 thread!", i);
            thread::sleep(Duration::from_millis(50));
        });
    }
    for i in 1..=5 {
        pool.execute(move || {
            println!("number {} from the  spawned_2 thread!", i);
            thread::sleep(Duration::from_millis(100));
        });
    }
    for i in 1..=5 {
        pool.execute(move || {
            println!("number {} from the  spawned_3 thread!", i);
            thread::sleep(Duration::from_millis(200));
        });
    }

    pool.join();
}
