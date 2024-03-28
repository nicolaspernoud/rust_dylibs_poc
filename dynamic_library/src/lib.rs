use std::{future::Future, pin::Pin, time::Duration};
use tokio::time::sleep;

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
fn library_task_future(id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> {
    println!("Started async code from library");
    Box::pin(library_task(id))
}
