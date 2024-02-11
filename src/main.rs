use std::process;
fn main() {
    let mut alloc_size = 1024 * 1024;

    // Initialize a vector to hold the allocated memory
    let mut vec: Vec<u8> = Vec::new();

    loop {
        match vec.try_reserve(alloc_size) {
            Ok(_) => {
                unsafe { vec.set_len(vec.len() + alloc_size); }
            }
            Err(_) => {
                println!("Failed to allocate memory, trying to decrease.");
                if alloc_size > 2 {
                    alloc_size = alloc_size / 2 ;
                }
                else {
                    break;
                }
            }
        }
    }

    process::exit(0);
}
