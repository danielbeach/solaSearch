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
    //println!("{:?}", content.await);
    let new_book: String = Book::remove_gutenberg_text(content.await);
    println!("{}", new_book);
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

    fn remove_gutenberg_text(raw_book: String) -> String {
        // We must strip away all references to Project Gutenberg to use this content as we wish.
        // This is legitimate and allowed by Project Gutenberg.
        let start_removed: &str = raw_book.split("*** START OF THIS PROJECT GUTENBERG EBOOK").nth(1).unwrap();
        let end_removed: &str= start_removed.split("End of the Project Gutenberg EBook").nth(0).unwrap();
        return end_removed.to_owned();
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
