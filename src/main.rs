mod scanner;

use self::scanner::token;

fn main() {
    let mut scanner = token::Scanner::new(String::from("a+b+\"cde\""));
    scanner.scan_tokens();
    for t in scanner {
        println!("{:?}", t);
    }
}
