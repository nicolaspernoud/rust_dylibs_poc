use libloading::{library_filename, Library, Symbol};

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

#[tokio::main]
async fn main() {
    match call_dynamic_simple(&"|Data from the main program|") {
        Ok(_) => println!("The dynamic library has been called !"),
        Err(e) => println!("{:?}", e),
    }
}
