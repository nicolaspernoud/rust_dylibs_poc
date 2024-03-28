use std::time::Duration;
use tokio::time::sleep;

#[no_mangle]
pub fn hello_from_library(data: &str) {
    println!(
        "Hello world from the dynamic library ! The caller program wants me to display {data}"
    );
}

#[tokio::main]
async fn library_task(id: usize) {
    let mut counter = 0;
    for _i in 0..5 {
        // Sleep for one second
        sleep(Duration::from_secs(1)).await;

        // Increment counter
        counter += 1;

        println!("From library : Task {}: Counter: {}", id, counter);
    }
}

#[no_mangle]
fn library_task_future(id: usize) {
    println!("Started async code from library");
    library_task(id)
}
