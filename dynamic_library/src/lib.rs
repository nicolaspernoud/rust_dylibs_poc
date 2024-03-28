use std::time::Duration;
use tokio::{runtime::Handle, time::sleep};

#[no_mangle]
fn library_task_future(id: usize, handle: Handle) -> tokio::task::JoinHandle<()> {
    println!("Started async code from library");
    handle.spawn(async move {
        println!("Started async code from counter");
        let mut counter = 0;
        for _i in 0..5 {
            // Sleep for one second
            sleep(Duration::from_secs(1)).await;

            // Increment counter
            counter += 1;

            println!("From library : Task {}: Counter: {}", id, counter);
        }
    })
}
