// Enum
enum Publication {
    Book(Book),
    Magazine(Magazine),
}

// Book Struct
struct Book {
    title: String,
    author: String,
    page_count: u16,
}

// Magazine Struct
struct Magazine {
    title: String,
    issue: u8,
    topic: String,
}

// Print Publication Function
fn print_publication(publication: Publication) {
    match publication {
        Publication::Book(book) => {
            println!("Book: {} Author: {}, Page Count {}",book.title, book.author, book.page_count);
        }
        Publication::Magazine(magazine) => {
            println!("Magazine: {} - Issue: {}, Topic: {}",magazine.title, magazine.issue, magazine.topic);
        }
    }
}

// I think its better to have a general function to print a publication
// Using this function in other situations will be easier too.
fn print_publications(publications:Vec<Publication>) {
    for publication in publications {
        print_publication(publication);
    }
}
fn main() {

    /*  Creating book instance  */
        let book1 = Book {
            title: String::from("Introdunction to Algorithms"),
            author: String::from("Ronald L. Rivest"),
            page_count: 1292,
        };
    /**/

    /*  Creating magazine instance */
        let magazine1 = Magazine {
            title: String::from("National Geographic"),
            issue: 202,
            topic: String::from("Climate Change"),
        };
    /**/

    /* Creating Publication instance with enum*/
        let publications: Vec<Publication> = vec![Publication::Book(book1), Publication::Magazine(magazine1)];
    /**/

    print_publications(publications);

}
