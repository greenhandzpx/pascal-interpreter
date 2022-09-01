
#[derive(PartialEq)]
enum OpType {
    INTEGER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    EOF,
}

struct Token {
    op_type: OpType,
    value: String,
}

impl Token {
    fn new(op_type: OpType, value: &str) -> Token {
        Token {
            op_type,
            value: String::from(value),
        }
    }
}

struct Lexer {
    text: String,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(text: String) -> Lexer {
        Lexer { pos: 0, current_char: Some(text.chars().nth(0).unwrap()), text }
    }
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos >= self.text.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.text.chars().nth(self.pos).unwrap());
        }
    }
    fn skip_space(&mut self) {
        while let Some(ch) = self.current_char && ch == ' ' {
            self.advance()
        }
    }
    fn integer_lexer(&mut self) -> String {
        let mut res = String::from("");
        while let Some(ch) = self.current_char && ch.is_digit(10) {
            res.push(ch);
            self.advance();
        }
        res
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            // println!("next token ch {}", ch);
            if ch.is_digit(10) {
                return Token::new(OpType::INTEGER, &self.integer_lexer());
            }
            match ch {
            ' ' => {
                self.skip_space();
                continue;
            }
            '+' => {
                self.advance();
                return Token::new(OpType::PLUS, "+")
            },
            '-' => { 
                self.advance();
                return Token::new(OpType::MINUS, "-")
            },
            '*' => { 
                self.advance();
                return Token::new(OpType::MUL, "*")
            },
            '/' => { 
                self.advance();
                return Token::new(OpType::DIV, "/")
            },
            '(' => { 
                self.advance();
                return Token::new(OpType::LPAREN, "(")
            },
            ')' => { 
                self.advance();
                return Token::new(OpType::RPAREN, ")")
            },
            '\n' => {
                break
            }
            _ => {
                panic!("unknown syntax {}", ch);
            }
            }
        }
        Token::new(OpType::EOF, "")
    }
}

struct Interpreter {
    lexer: Lexer,
    current_token: Token,
}

impl Interpreter {
    fn new(mut lexer: Lexer) -> Interpreter {
        Interpreter {
            current_token: lexer.get_next_token(),
            lexer
        }
    }
    fn eat(&mut self, op_type: OpType) {
        // println!("eat: old current token {}", self.current_token.value);
        if self.current_token.op_type == op_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("unknown syntax")
        }
        // println!("eat: new current token {}", self.current_token.value);
    }
    fn factor(&mut self) -> i32 {
        match self.current_token.op_type {
        OpType::INTEGER => {
            let res = self.current_token.value.parse::<i32>().unwrap();
            self.eat(OpType::INTEGER);
            return res;
        },
        OpType::LPAREN => {
            self.eat(OpType::LPAREN);
            let res = self.expr();
            self.eat(OpType::RPAREN);
            res
        },
        _ => panic!("syntax error")
        }
    }
    fn term(&mut self) -> i32 {
        let mut res = self.factor();
        while self.current_token.op_type == OpType::MUL ||
            self.current_token.op_type == OpType::DIV {
            match self.current_token.op_type {
            OpType::MUL => {
                self.eat(OpType::MUL);
                res *= self.factor();
            },
            OpType::DIV => {
                self.eat(OpType::DIV);
                res /= self.factor();
            }
            _ => ()
            }
        }
        res
    }
    fn expr(&mut self) -> i32 {
        let mut res = self.term();
        while self.current_token.op_type == OpType::PLUS ||
            self.current_token.op_type == OpType::MINUS {
            match self.current_token.op_type {
            OpType::PLUS => {
                self.eat(OpType::PLUS);
                res += self.term();
            }
            OpType::MINUS => {
                self.eat(OpType::MINUS);
                res -= self.term();
            }
            _ => ()
            }
        }
        res
    }
}


fn main() {
    loop {
        print!("calc> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut text = String::new();
        std::io::stdin().read_line(&mut text).unwrap();
        let lexer = Lexer::new(text);
        let mut interpreter = Interpreter::new(lexer);
        let res = interpreter.expr();
        print!("{}\n", res);
    }
}
