use std::sync::atomic::{AtomicUsize, Ordering};

static Q: AtomicUsize = AtomicUsize::new(0);

pub fn q() {
    println!("Q: {}", Q.fetch_add(1, Ordering::Relaxed) + 1);
}

pub fn qr() {
    println!("Total: {}\n", Q.swap(0, Ordering::Relaxed));
}