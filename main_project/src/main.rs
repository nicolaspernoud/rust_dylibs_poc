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

fn call_dynamic_simple(function_parameter: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new(format!(
            "./{}",
            library_filename("dynamic_library").into_string().unwrap()
        ))?;
        let func: Symbol<fn(&str)> = lib.get(b"hello_from_library")?;
        Ok(func(function_parameter))
    }
}

async fn call_dynamic_tokio(id: usize) {
    unsafe {
        let lib = Library::new(format!(
            "./{}",
            library_filename("dynamic_library").into_string().unwrap()
        ))
        .expect("could not load library");
        let future: Symbol<fn(id: usize)> = lib
            .get(b"library_task_future")
            .expect("could not load function from library");
        future(id);
    }
}

#[tokio::main]
async fn main() {
    match call_dynamic_simple(&"|Data from the main program|") {
        Ok(_) => println!("The dynamic library has been called !"),
        Err(e) => println!("{:?}", e),
    }

    // Spawn two tasks
    let task1 = spawn_local_task(1);
    call_dynamic_tokio(2).await;

    // Wait for main task to complete
    task1.await.expect("main program terminated with an error");
}
