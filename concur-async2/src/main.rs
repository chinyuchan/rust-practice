use futures::executor::block_on;
use futures::join;
struct DataStructure;
struct Algorithm {
    ds: DataStructure,
}

async fn learn_rust() {
    println!("Learn Rust!");
}

async fn learn_data_structure() -> DataStructure {
    println!("Learn data structure!");
    DataStructure
}

async fn learn_algorithm(ds: DataStructure) -> Algorithm {
    println!("Learn algorithm!");
    Algorithm { ds }
}

async fn learn_data_structure_and_algorithm() -> Algorithm {
    // await不会阻塞当前线程，如果函数阻塞，可让其他future来接接管当前线程
    let ds = learn_data_structure().await;
    learn_algorithm(ds).await
}

async fn async_main() {
    let future1 = learn_data_structure_and_algorithm();
    let future2 = learn_rust();
    join!(future1, future2);
}

fn main() {
    block_on(async_main());
}
