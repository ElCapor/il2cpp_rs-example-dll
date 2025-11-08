# Example DLL for il2cpp_rs
This is a simple example DLL for [il2cpp_rs](https://github.com/ElCapor/il2cpp_rs).

# Building
- Install and build:

```bash
rustup toolchain install stable
cargo build
```

- Standard dev build:

```bash
cargo build
```

The library compiles as a DLL (due to `DllMain`). You’ll typically inject it into a running IL2CPP process on Windows.

# Running
- On DLL load, `DllMain` spawns a Rust thread and calls `entry_point()`.
- `entry_point()`:
  - Allocates a console and initializes the IL2CPP API
  - Attaches the thread to the IL2CPP domain
  - Builds the `Cache` by walking assemblies → classes → fields/methods
  - Prints debug info (you can comment/uncomment the debug prints)

# Entry Point
```rust
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
```
