use il2cpp_rs::{console, il2cpp, il2cpp_cache, profile_call};
use std::process::exit;
use std::thread;

pub fn entry_point() {
    // Initialize the console
    if let Err(e) = console::allocate_console() {
        println!("Error: {}", e);
        exit(-1);
    }
    println!("Initializing Il2CppApi...");
    match il2cpp::init("GameAssembly.dll") {
        Ok(_) => {
            println!("Il2CppApi initialized");
        }
        Err(e) => {
            println!("Error: {}", e);
            console::wait_line_press_to_exit(-1);
        }
    }

    match il2cpp::get_domain() {
        Ok(domain) => {
            println!("Domain: {:p}", domain);
            let _ = il2cpp::thread_attach(domain);
            println!("Attached to domain");
            let cache = profile_call!("Cache::new", il2cpp_cache::Cache::new(domain));
            match cache {
                Ok(cache) => {
                    //println!("{:?}", cache);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    console::wait_line_press_to_exit(-1);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            console::wait_line_press_to_exit(-1);
        }
    }

    il2cpp::print_all_function_ptrs();
    console::wait_line_press_to_exit(-1);
}

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: usize, reason: u32, _: usize) -> i32 {
    match reason {
        1 => {
            let _ = thread::spawn(|| {
                entry_point();
            });
        }
        _ => {}
    }
    1
}
