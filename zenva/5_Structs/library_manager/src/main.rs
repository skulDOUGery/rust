/* Step 1: Define a Book struct */
struct Book {
    title: String,
    author: String,
    number_of_pages: u32,
}

/* Step 2: Implement the get_summary method on the Book struct */
impl Book {
    fn get_summary(&self) -> String {
        format!("{}\n\tAuthor: \t\t{}\n\tNumber of Pages:\t{}\n",
            self.title,
            self.author, 
            self.number_of_pages)
    }
}

fn main() {

    /* Step 3: Create an array of books */
    let books: [Book; 5] = [
        Book {
            title: String::from("To Kill a Mockingbird"),
            author: String::from("Harper Lee"),
            number_of_pages: 320,
        },
        
        Book {
            title: String::from("Le Morte d'Arthur"),
            author: String::from("Sir Thomas Mallory"),
            number_of_pages: 389,
        },
        Book {
            title: String::from("The Fairytale Ends: A Tale of Gawayne and the Green Knight"),
            author: String::from("Cat Russell"),
            number_of_pages: 81,
        },
        Book {
            title: String::from("Kaleidoscope: The Poetry of Cat Russell"),
            author: String::from("Cat Russell"),
            number_of_pages: 114,
        },
        Book {
            title: String::from("An Optimist's Journal of the End of Days and Other Stories"),
            author: String::from("Cat Russell"),
            number_of_pages: 224,
        },
    ];

    /* 
        Step 4: Use a for loop to iterate through the array 
        and print the summary of each book
    */
    for book in books {
        println!("{}", book.get_summary());
    }
}
