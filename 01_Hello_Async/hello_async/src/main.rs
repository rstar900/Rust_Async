
// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread

use futures::executor::block_on;

// async keyword is important to mark that the function will return a future
// which can then be executed by an executor
async fn hello() {
    println!("Hello, Async World!");
}

fn main() {
    let hello_future = hello(); // Nothing is printed yet, only a future is returned
    block_on(hello_future); //`future` is run and "Hello, Async World!" is printed
}
