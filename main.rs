
fn main() {
    println!("Hello, world!");
}

impl Book {
    // methods to help work on Books.
    // work on text files, split into chapters, paragraphs, sentences.
}

impl Author {
    // methods to help work on Authors
}
struct Book {
    // Everything needed for a book.
    author_id: u32,
    title: String,
    publish_year: u32,
    book_id: u32,
    period: TimePeriod
}

struct Author {
    // What we need for authors.
    author_id: u32,
    first_name: String,
    last_name: String,
    year_born: u32,
    year_death: u32
}

enum TimePeriod {
    EarlyChurch,
    Reformer,
    Puritan,
    Nineteenth_Century,

}

struct GutenbergDownloader {
    // what we need to know about Gutenberg
}

impl GutenbergDownloader {
    // retrieve Gutenberg Files
}

struct Elastic {
    // what we need to know about ElasticSearch
}

impl Elastic {
    // How to ingest Books into ElasticSearch
}