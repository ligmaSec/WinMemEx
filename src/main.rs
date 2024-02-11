use std::mem;
use std::process;
use std::time::Duration;

fn main() {
    // Define the size of each allocation and the total amount of memory to consume
    let alloc_size =  1024 *  1024; //  1 MB
    let total_memory_to_consume =  1024 *  1024; //  1 GB

    // Initialize a vector to hold the allocated memory
    let mut vec: Vec<u8> = Vec::new();

    // Start a loop to allocate memory
    loop {
        // Attempt to allocate memory
        match vec.try_reserve(alloc_size) {
            Ok(_) => {
                // Successfully allocated memory, add it to the vector
                unsafe { vec.set_len(vec.len() + alloc_size); }
            }
            Err(_) => {
                // Failed to allocate memory, exit the loop
                println!("Failed to allocate memory.");
                break;
            }
        }

        // Optional: sleep for a short duration to prevent high CPU usage
        //std::thread::sleep(Duration::from_millis(10));
    }

    // Optionally print out the final memory consumption
    println!("Final memory consumption: {} bytes", vec.capacity());

    // Exit the program gracefully
    process::exit(0);
}
