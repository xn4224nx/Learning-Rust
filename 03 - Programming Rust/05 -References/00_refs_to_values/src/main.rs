use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show_tbl(table: &Table) {

    for (artist, works) in table {
    
        println!("\nWorks by {}:", artist);
        for book in works {
            println!("\t{}", book);
        }
    }
}

fn main() {
    
    let mut authors = Table::new();
    
    authors.insert("J. K. Rowling".to_string(), vec![
        "Harry Potter and the Philosopher's Stone".to_string(),
        "Harry Potter and the Chamber of Secrets".to_string(),
        "Harry Potter and the Prisoner of Azkaban".to_string(),
        "Harry Potter and the Goblet of Fire".to_string(),
    ]);
    
    authors.insert("J. R. R. Tolkien".to_string(),vec![
        "The Hobbit".to_string(),
        "The Fellowship of the Ring".to_string(),
        "The Two Towers".to_string(),
        "The Return of the King".to_string(),
    ]);
    
    authors.insert("Hilary Mantel".to_string(), vec![
        "Wolf Hall".to_string(),
        "Bring Up the Bodies".to_string(),
        "The Mirror and the Light".to_string(),
        "Beyond Black".to_string(),
    ]);
    
    show_tbl(&authors);
}
