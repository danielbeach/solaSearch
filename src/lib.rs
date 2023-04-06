use reqwest;

pub enum TimePeriod {
    EarlyChurch,
    Reformer,
    Puritan,
    NineteenthCentury,

}
pub struct Book {
    // Everything needed for a book.
    pub author_id: u32,
    pub title: String,
    pub publish_year: u32,
    pub book_id: u32,
    pub period: TimePeriod,
    pub uri: String
}

#[derive(Debug)]
pub struct Paragraph {
    paragraph_id : u32,
    paragraph: String
}


impl Book {
    // methods to help work on Books.
    pub async fn download_book(&self) -> String {
        let full_uri: String = format!("{}{}/{}{}", &self.uri, self.book_id.to_string(), self.book_id.to_string(), ".txt".to_string());
        let content: String = reqwest::get(full_uri)
        .await.unwrap()
        .text()
        .await.unwrap();
        return content;
    }

    pub fn remove_gutenberg_text(raw_book: String) -> String {
        // We must strip away all references to Project Gutenberg to use this content as we wish.
        // This is legitimate and allowed by Project Gutenberg.
        let start_removed: &str = raw_book.split("*** START OF THIS PROJECT GUTENBERG EBOOK").nth(1).unwrap();
        let end_removed: &str= start_removed.split("End of the Project Gutenberg EBook").nth(0).unwrap();
        return end_removed.to_owned();
    }

    // write a public function to split a string into a vector of paragraphs by blank lines.
    // This will be used to create a vector of paragraphs for each book.
    pub fn split_into_paragraphs(book: String) -> Vec<Paragraph> {
        let mut paragraphs: Vec<Paragraph> = Vec::new();
        let mut paragraph: String = String::new();
        let mut paragraph_counter: u32 = 1;
        for line in book.lines() {
            if line == "" {
                let paragraph_data: Paragraph = Paragraph {
                    paragraph_id: paragraph_counter,
                    paragraph: paragraph
                };
                if &paragraph_data.paragraph != "" {
                    paragraphs.push(paragraph_data);
                }
                paragraph = String::new();
            } else {
                paragraph.push_str(line);
                paragraph.push_str(" ");
            }
            paragraph_counter += 1;
        }
        return paragraphs;
    }
}

pub struct Author {
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

pub struct Elastic {
    // what we need to know about ElasticSearch
}