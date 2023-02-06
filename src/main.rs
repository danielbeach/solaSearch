
fn main() {
    println!("Hello, world!");
}

impl Book {
    
}
struct Book {
    author: String,
    title: String,
    publish_year: u32,
    book_id: u32,
    period: TimePeriod
}

enum TimePeriod {
    EarlyChurch,
    Reformer,
    Puritan,
}