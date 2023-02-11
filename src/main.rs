use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let book = Book {
        author_id: 1,
        title: "The Confessions of St. Augustine".to_string(),
        publish_year: 397,
        book_id: 3296,
        period: TimePeriod::EarlyChurch,
        uri: "https://www.gutenberg.org/files/".to_string()
    };
    let content = book.download_book();
    println!("{:?}", content.await);
    Ok(())
}

struct Book {
    // Everything needed for a book.
    author_id: u32,
    title: String,
    publish_year: u32,
    book_id: u32,
    period: TimePeriod,
    uri: String
}

impl Book {
    // methods to help work on Books.
    async fn download_book(&self) -> String {
        let full_uri = format!("{}{}/{}{}", &self.uri, self.book_id.to_string(), self.book_id.to_string(), ".txt".to_string());
        let content = reqwest::get(full_uri)
        .await.unwrap()
        .text()
        .await.unwrap();
        return content;
    }
}

struct Author {
    // What we need for authors.
    author_id: u32,
    first_name: String,
    last_name: String,
    year_born: u32,
    year_death: u32,
    book_id :u32
}

impl Author {
    // methods to help work on Authors
}

enum TimePeriod {
    EarlyChurch,
    Reformer,
    Puritan,
    NineteenthCentury,

}

struct Elastic {
    // what we need to know about ElasticSearch
}

impl Elastic {
    // How to ingest Books into ElasticSearch
}
