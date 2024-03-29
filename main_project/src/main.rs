use libloading::{library_filename, Library, Symbol};
use std::future::Future;
use std::pin::Pin;
use tokio::time::{sleep, Duration};

async fn call_dynamic_tokio(id: usize) -> tokio::task::JoinHandle<()> {
    tokio::spawn({
        unsafe {
            let lib = Library::new(format!(
                "./{}",
                library_filename("dynamic_library").into_string().unwrap()
            ))
            .expect("could not load library");
            let future: Symbol<fn(id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>> =
                lib.get(b"library_task_future")
                    .expect("could not load function from library");
            future(id)
        }
    })
}

#[tokio::main]
async fn main() {
    // Spawn two tasks
    tokio::select! {
        _ = call_local_tokio(1) => {
            println!("call_local_tokio() completed first")
        }
        _ = call_dynamic_tokio(2) => {
            println!("call_dynamic_tokio() completed first")
        }
    };
}

fn call_local_tokio(id: usize) -> tokio::task::JoinHandle<()> {
    tokio::spawn(local_task_future(id))
}

async fn local_task(id: usize) {
    let mut counter = 0;
    for _i in 0..5 {
        // Sleep for one second
        sleep(Duration::from_secs(1)).await;

        // Increment counter
        counter += 1;

        println!("Locally: Task {}: Counter: {}", id, counter);
    }
}

fn local_task_future(id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> {
    println!("Started async code locally");
    Box::pin(local_task(id))
}
