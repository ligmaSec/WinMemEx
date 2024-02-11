
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use std::process;
use rand::Rng;
//use std::time::Duration;
//use std::thread;
const ALLOCATION_SIZE: usize =  1024 * 1024; //  1 MB

fn main() {
    let mut allocations = Vec::new();
    
    
    let mut sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
    let mut rng = rand::thread_rng();
    loop {
        match check_free_memory(&mut sys) {
            Ok(free_memory) => {
                if free_memory >= ALLOCATION_SIZE {
                    allocations.push(vec![rng.gen_range(0..255); ALLOCATION_SIZE]);
                    println!("{} Bytes free", free_memory);
                } else {
                    println!("Warning: Less than  1 MB of free memory left.");
                }
            },
            Err(e) => {
                eprintln!("An error occured while trying to fetch the remaining memory: {}", e);
                process::exit(1);
            }
        }

        //thread::sleep(Duration::from_millis(100));
    }
}

fn check_free_memory(sys: &mut System) -> Result<usize, Box<dyn std::error::Error>> {

    sys.refresh_memory();
    Ok(sys.free_memory() as usize)


}
