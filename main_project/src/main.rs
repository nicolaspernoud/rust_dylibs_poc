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

fn spawn_dynamic_task(id: usize) -> JoinHandle<()> {
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
    tokio::select! {
        _ = spawn_local_task(1) => {
            println!("spawn_local_task() completed first")
        }
        _ = spawn_dynamic_task(2) => {
            println!("spawn_dynamic_task() completed first")
        }
    };
}
