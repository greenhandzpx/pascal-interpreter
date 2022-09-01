use std::rc::Rc;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
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

trait AstNode {
    fn get_op_type(&self) -> OpType;
    // not sure return type
    fn get_left(&self) -> Option<Rc<dyn AstNode>> {
        None
    }
    fn get_right(&self) -> Option<Rc<dyn AstNode>> {
        None
    }
    fn get_value(&self) -> Option<i32> {
        None
    }
}

struct BinOp {
    op_type: OpType,
    left: Rc<dyn AstNode>,
    right:Rc<dyn AstNode>,
}
impl AstNode for BinOp {
    fn get_op_type(&self) -> OpType {
        self.op_type 
    }
    fn get_left(&self) -> Option<Rc<dyn AstNode>> {
        Some(self.left.clone())
    }
    fn get_right(&self) -> Option<Rc<dyn AstNode>> {
        Some(self.right.clone())
    }
}
impl BinOp {
    fn new(op_type: OpType, left: Rc<dyn AstNode>, right: Rc<dyn AstNode>) ->BinOp {
        BinOp { op_type, left, right }
    }
}

struct Num {
    op_type: OpType,
    value: i32,
}
impl AstNode for Num {
    fn get_op_type(&self) -> OpType {
        self.op_type 
    }
    fn get_value(&self) -> Option<i32> {
        Some(self.value)
    }
}
impl Num {
    fn new(op_type: OpType, value: i32) -> Num {
        Num {
            value,
            op_type,
        }
    }
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

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        Parser {
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
    fn factor(&mut self) -> Rc<dyn AstNode> {
        match self.current_token.op_type {
        OpType::INTEGER => {
            self.eat(OpType::INTEGER);
            Rc::new(Num::new(self.current_token.op_type, self.current_token.value.parse::<i32>().unwrap()))
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
    fn term(&mut self) -> Rc<dyn AstNode> {
        let mut node = self.factor();
        while self.current_token.op_type == OpType::MUL ||
            self.current_token.op_type == OpType::DIV {

            match self.current_token.op_type {
                OpType::MUL => {
                    self.eat(OpType::MUL);
                },
                OpType::DIV => {
                    self.eat(OpType::DIV);
                }
                _ => ()
            }
            let op_type = self.current_token.op_type;
            // we construct the tree from bottom to top
            node = Rc::new(BinOp::new(op_type, node, self.factor()));
        }
        node
    }
    fn expr(&mut self) -> Rc<dyn AstNode> {
        let mut node = self.term();
        while self.current_token.op_type == OpType::PLUS ||
            self.current_token.op_type == OpType::MINUS {

            match self.current_token.op_type {
                OpType::PLUS => {
                    self.eat(OpType::PLUS);
                }
                OpType::MINUS => {
                    self.eat(OpType::MINUS);
                }
                _ => ()
            }

            // we construct the tree from bottom to top
            let op_type = self.current_token.op_type;
            node = Rc::new(BinOp::new(op_type, node, self.term()));
        }
        node
    }
}

trait NodeVisitor {
    fn visit(&self, node: Rc<dyn AstNode>) -> i32 {
        0 
        // TODO: try to invoke the right function according to the type of the node
    }
}

struct Interpreter {
    parser: Parser,
}
impl NodeVisitor for Interpreter {}
impl Interpreter {
    fn new(parser: Parser) -> Interpreter {
        Interpreter { parser }
    }

    fn visit_BinOp(&self, node: &dyn AstNode) -> i32 {
        match node.get_op_type() {
            OpType::PLUS => self.visit(node.get_left().unwrap()) + self.visit(node.get_right().unwrap()),
            OpType::MINUS => self.visit(node.get_left().unwrap()) - self.visit(node.get_right().unwrap()),
            OpType::MUL => self.visit(node.get_left().unwrap()) * self.visit(node.get_right().unwrap()),
            OpType::DIV => self.visit(node.get_left().unwrap()) / self.visit(node.get_right().unwrap()),
            _ => panic!("error syntax")
        } 
    }
    fn visit_Num(&self, node: &dyn AstNode) -> i32 {
        node.get_value().unwrap()
    }
}

