use futures::executor::block_on;

async fn hello() {
    println!("Hello, async!");
}

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

fn main() {
    let fut = hello();
    block_on(fut);

    let ds = block_on(learn_data_structure());
    block_on(learn_algorithm(ds));
    block_on(learn_rust())
}
