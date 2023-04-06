use gutenberg::Book;
use gutenberg::TimePeriod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let book: Book = Book {
        author_id: 1,
        title: "The Confessions of St. Augustine".to_string(),
        publish_year: 397,
        book_id: 3296,
        period: TimePeriod::EarlyChurch,
        uri: "https://www.gutenberg.org/files/".to_string()
    };
    let content = book.download_book();
    //println!("{:?}", content.await);
    let new_book: String = gutenberg::Book::remove_gutenberg_text(content.await);
    let paragraphs: Vec<gutenberg::Paragraph> = gutenberg::Book::split_into_paragraphs(new_book);
    for paragrah in paragraphs {
        println!("{:?}", paragrah);
    }
    Ok(())
}
