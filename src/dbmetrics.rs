use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

static QUERY_COUNT: AtomicUsize = AtomicUsize::new(0);
static REQUEST_START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub fn start_request() {
    QUERY_COUNT.store(0, Ordering::SeqCst);
    REQUEST_START.get_or_init(|| Instant::now());
}

pub fn q() {
    let count = QUERY_COUNT.fetch_add(1, Ordering::SeqCst);
    
    eprintln!("  Query #{} ({}ms)", count + 1, 
        REQUEST_START.get().map(|s| s.elapsed().as_millis()).unwrap_or(0));
}

pub fn qr() -> (usize, u128) {
    let count = QUERY_COUNT.load(Ordering::SeqCst);
    let elapsed = REQUEST_START
        .get()
        .map(|start| start.elapsed().as_millis())
        .unwrap_or(0);
    
    
    let dataloader_status = std::env::var("USE_DATALOADER")
        .unwrap_or_else(|_| "true".to_string());
    let status = if dataloader_status == "true" { "✅ ENABLED " } else { "❌ DISABLED" };
    
    println!("\n┌────────────────────────────────────────┐");
    println!("│  📊 GraphQL Request Metrics            │");
    println!("├────────────────────────────────────────┤");
    println!("│  Total DB Queries: {:>4}               │", count);
    println!("│  Request Duration: {:>4}ms             │", elapsed);
    if count > 0 {
        println!("│  Avg per query:    {:>4}ms             │", elapsed / count as u128);
    }
    println!("│  DataLoader:       {}         │", status);
    println!("└────────────────────────────────────────┘\n");
    
    (count, elapsed)
}