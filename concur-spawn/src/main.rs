use std::thread;
use std::time::Duration;

fn main() {
    let thread1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_1 thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    let thread2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_2 thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    for i in 1..=5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }
    thread1.join().unwrap();
    thread2.join().unwrap();

    let v = vec![1, 2, 3, 4, 5];
    let h = thread::spawn(move || {
        println!("{:?}", v);
    });
    h.join().unwrap();
}
