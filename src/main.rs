extern crate zodiac_parsing;
use zodiac_parsing::lexing::Lexer;

fn main() {
    let lexer = Lexer::parse("
        <rect large-size=\"true\" rounded-edges=\"true\" other>
            <line>
            </line>
        </rect>");
    for token in lexer {
        match token {
            Ok(value) => println!("{:?}", value),
            Err(error) => println!("{:?}", error) 
        }
    }
}
