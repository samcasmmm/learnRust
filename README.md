learning Rust zero to hero


Concurrency
Concurrency: Rust provides safe concurrency using threads and message passing with the std::thread module and std::sync primitives.
rust

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```
