use std::sync::atomic::{AtomicI64, Ordering};

pub static CONNECTION_COUNTER: AtomicI64 = AtomicI64::new(0);

pub fn mut_to_connection_counter(num: i64) {
    CONNECTION_COUNTER.fetch_add(num, Ordering::Relaxed);
}

pub fn get_connection_counter() -> i64 {
    CONNECTION_COUNTER.load(Ordering::Relaxed)
}

pub fn print_connection_counter() {
    println!("Connections: {}", get_connection_counter());
}
