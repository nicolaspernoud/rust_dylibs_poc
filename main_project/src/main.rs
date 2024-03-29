use libloading::{library_filename, Library, Symbol};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

async fn local_task(id: usize) {
    let mut counter = 0;

    for _i in 0..5 {
        sleep(Duration::from_secs(1)).await;
        counter += 1;
        println!("Task {}: Counter: {}", id, counter);
    }
}

fn spawn_local_task(id: usize) -> JoinHandle<()> {
    tokio::spawn(local_task(id))
}

fn spawn_remote_task(id: usize) -> JoinHandle<()> {
    tokio::spawn(async move {
        unsafe {
            let lib = Library::new(format!(
                "./{}",
                library_filename("dynamic_library").into_string().unwrap()
            ))
            .expect("could not load library");
            let future: Symbol<fn(id: usize)> = lib
                .get(b"library_task_future")
                .expect("could not load function from library");
            future(id)
        }
    })
}

#[tokio::main]
async fn main() {
    // Spawn two tasks
    let task1 = spawn_local_task(1);
    let _task2 = spawn_remote_task(2);

    // Wait for main task to complete
    task1.await.expect("main program terminated with an error");
}
