//
// Authors:
// Binh Le
//

#[derive(Debug)]
enum Token {
    //  Operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    // Punctuators
    LParen,
    RParen,
    Equal,
    Semicolon,
    Let,

    Identifier(String),
    NumberLiteral(f64),

    //Special types
    End,
    Illegal(char),
}

struct Lexer {
    input: String,
    cursor: usize,
    current_char: char,
}

impl Lexer {
    fn read_char(&mut self) {
        if self.cursor >= self.input.len() {
            self.current_char = '0';
        } else {
            self.current_char = match self.input.chars().nth(self.cursor) {
                Some(c) => c,
                None => '0',
            };
            self.cursor += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.read_char();
        }
    }

    fn create_lexer(text: &String) -> Lexer {
        let mut new_lexer = Lexer {
            input: text.to_string(),
            cursor: 0,
            current_char: '0',
        };
        &new_lexer.read_char();
        return new_lexer;
    }

    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();

        let new_token: Token;

        match self.current_char {
            '+' => new_token = Token::Plus,
            '-' => new_token = Token::Minus,
            '*' => new_token = Token::Star,
            '/' => new_token = Token::Slash,
            '^' => new_token = Token::Caret,
            '(' => new_token = Token::LParen,
            ')' => new_token = Token::RParen,
            ';' => new_token = Token::Semicolon,
            '=' => new_token = Token::Equal,
            'a'..='z' => new_token = Token::Identifier(self.current_char.to_string()),
            '0'..='9' => new_token = Token::NumberLiteral( self.current_char.to_string().parse().unwrap()),
            
            _ => new_token = Token::Illegal(self.current_char),
        }

        self.read_char();
        return new_token;
    }
}

fn read_file_contents(filename: &String) -> Result<String, String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = match File::open(filename) {
        Ok(file_object) => file_object,
        Err(msg) => {
            dbg!(msg);
            return Err("Couldn't open file.".to_string());
        }
    };

    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(msg) => {
            dbg!(msg);
            return Err("Could not read file contents.".to_string());
        }
    };

    return Ok(file_contents);
}

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let filename: &String;

    if args.len() == 2 {
        filename = &args[1];
    } else {
        panic!("ERROR: Incorrect number of arguments. Try ./tokens {filename}")
    }

    let file_contents = match read_file_contents(&filename) {
        Ok(s) => s,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut lexer = Lexer::create_lexer(&file_contents);

    loop {
        let token: Token = lexer.get_next_token();
        match token {
            Token::End => break,
            Token::Illegal(t) => panic!("ERROR: Unexpected character: '{}'", t ),
            _ => println!("{:?}", token),
        }
    }
}
