#[no_mangle]
pub fn hello_from_library(data: &str) {
    println!(
        "Hello world from the dynamic library ! The caller program wants me to display {data}"
    );
}
