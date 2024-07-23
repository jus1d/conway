use std::fs;

mod lexer;

fn main() {
    let filepath: String = String::from("./conway.rs");
    let source = fs::read_to_string(filepath.clone()).unwrap();

    let tokens = lexer::Lexer::from_iter(source.chars());
    
    let mut count: u64 = 0;
    for token in tokens {
        println!("{:?}", token);
        count += 1;
    }

    println!("Total tokens: {count}");
}
