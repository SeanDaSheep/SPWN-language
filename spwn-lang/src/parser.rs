use crate::ast;
/*use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;*/

use crate::builtin::Builtin;

use crate::compiler::ErrorReport;
use crate::compiler::RainbowColorGenerator;
use crate::compiler_info::CodeArea;

//use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use ariadne::Fmt;
use internment::Intern;
//use ast::ValueLiteral;
use logos::Lexer;
use logos::Logos;

use crate::compiler::create_error;
use crate::compiler_types::ImportType;

pub type FileRange = (usize, usize);

macro_rules! expected {
    ($expected:expr, $tokens:expr, $notes:expr, $a:expr) => {
        return Err(SyntaxError::ExpectedErr {
            expected: $expected,
            found: format!(
                "{}: \"{}\"",
                match $a {
                    Some(t) => t.typ(),
                    None => "EOF",
                },
                $tokens.slice()
            ),
            pos: $tokens.position(),
            file: $notes.file.clone(),
        });
    };
}

#[derive(Debug)]
pub enum SyntaxError {
    ExpectedErr {
        expected: String,
        found: String,
        pos: FileRange,
        file: PathBuf,
    },
    UnexpectedErr {
        found: String,
        pos: FileRange,
        file: PathBuf,
    },
    SyntaxError {
        message: String,
        pos: FileRange,
        file: PathBuf,
    },
}

pub fn is_valid_symbol(name: &str, tokens: &Tokens, notes: &ParseNotes) -> Result<(), SyntaxError> {
    if name.starts_with('_') && name.ends_with('_') {
        if Builtin::from_str(name).is_ok() {
            Ok(())
        } else {
            Err(SyntaxError::SyntaxError {
                message: format!("{} is an invalid variable/property/argument name", name),
                pos: tokens.position(),
                file: notes.file.clone(),
            })
        }
    } else {
        Ok(())
    }
}

impl From<SyntaxError> for ErrorReport {
    fn from(err: SyntaxError) -> ErrorReport {
        use crate::compiler_info::CompilerInfo;
        //write!(f, "SuperErrorSideKick is here!")
        let mut colors = RainbowColorGenerator::new(60.0, 1.0, 0.8);
        let a = colors.next();
        let b = colors.next();
        match err {
            SyntaxError::ExpectedErr {
                expected,
                found,
                pos,
                file,
            } => create_error(
                CompilerInfo::from_area(CodeArea {
                    pos,
                    file: Intern::new(file.clone()),
                }),
                "Syntax error",
                &[(
                    CodeArea {
                        pos,
                        file: Intern::new(file),
                    },
                    &format!(
                        "{} {}, {} {}",
                        "Expected".fg(b),
                        expected,
                        "found".fg(a),
                        found
                    ),
                )],
                None,
            ),

            SyntaxError::UnexpectedErr { found, pos, file } => create_error(
                CompilerInfo::from_area(CodeArea {
                    pos,
                    file: Intern::new(file.clone()),
                }),
                "Syntax error",
                &[(
                    CodeArea {
                        pos,
                        file: Intern::new(file),
                    },
                    &format!("Unexpected {}", found),
                )],
                None,
            ),

            SyntaxError::SyntaxError { message, pos, file } => create_error(
                CompilerInfo::from_area(CodeArea {
                    pos,
                    file: Intern::new(file.clone()),
                }),
                "Syntax error",
                &[(
                    CodeArea {
                        pos,
                        file: Intern::new(file),
                    },
                    &message,
                )],
                None,
            ),
        }
    }
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token {
    //OPERATORS
    #[token("->")]
    Arrow,

    #[token("=>")]
    ThickArrow,

    #[token("<=>")]
    Swap,

    #[token("|")]
    Either,

    #[token("||")]
    Or,

    #[token("&&")]
    And,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token(">=")]
    MoreOrEqual,

    #[token("<=")]
    LessOrEqual,

    #[token(">")]
    MoreThan,

    #[token("<")]
    LessThan,

    #[token("*")]
    Star,

    #[token("%")]
    Modulo,

    #[token("^")]
    Power,

    #[token("**")]
    DoubleStar,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token("/%")]
    IntDividedBy,

    #[token("has")]
    Has,

    #[token("!")]
    Exclamation,

    #[token("=")]
    Assign,

    #[token("+=")]
    Add,
    #[token("-=")]
    Subtract,
    #[token("*=")]
    Multiply,
    #[token("/=")]
    Divide,

    #[token("/%=")]
    IntDivide,

    #[token("^=")]
    Exponate,

    #[token("%=")]
    Modulate,

    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,

    #[token("as")]
    As,

    //VALUES
    #[regex(r"([a-zA-Z_][a-zA-Z0-9_]*)|\$")]
    Symbol,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex(r#""(?:\\.|[^\\"])*"|'(?:\\.|[^\\'])*'"#)]
    StringLiteral,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[regex(r"[0-9?]+[gbci]")]
    Id,

    //TERMINATORS
    #[token(",")]
    Comma,

    #[token("{")]
    OpenCurlyBracket,

    #[token("}")]
    ClosingCurlyBracket,

    #[token("[")]
    OpenSquareBracket,

    #[token("]")]
    ClosingSquareBracket,

    #[token("(")]
    OpenBracket,

    #[token(")")]
    ClosingBracket,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token(".")]
    Period,

    #[token("..")]
    DotDot,

    #[token("@")]
    At,

    #[token("#")]
    Hash,

    //KEY WORDS
    #[token("return")]
    Return,

    /*#[token("<+")]
    Add,*/
    #[token("impl")]
    Implement,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("throw")]
    ErrorStatement,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("switch")]
    Switch,

    #[token("case")]
    Case,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("while")]
    While,

    #[token("obj")]
    Object,

    #[token("trigger")]
    Trigger,

    #[token("import")]
    Import,

    #[token("extract")]
    Extract,

    #[token("null")]
    Null,

    #[token("type")]
    Type,

    #[token("let")]
    Let,

    #[token("self")]
    SelfVal,

    #[token("sync")]
    Sync,

    //STATEMENT SEPARATOR
    #[regex(r"[\n\r;]+")]
    StatementSeparator,

    #[error]
    #[regex(r"[ \t\f]+|/\*[^*]*\*(([^/\*][^\*]*)?\*)*/|//[^\n]*", logos::skip)]
    Error,
}

impl Token {
    fn typ(&self) -> &'static str {
        use Token::*;
        match self {
            Or | And | Equal | NotEqual | MoreOrEqual | LessOrEqual | MoreThan | LessThan
            | Star | Modulo | Power | Plus | Minus | Slash | Exclamation | Assign | Add
            | Subtract | Multiply | Divide | IntDividedBy | IntDivide | As | Has | Either
            | DoubleStar | Exponate | Modulate | Increment | Decrement | Swap => "operator",
            Symbol => "identifier",
            Number => "number literal",
            StringLiteral => "string literal",
            True | False => "boolean literal",
            Id => "ID literal",

            Comma | OpenCurlyBracket | ClosingCurlyBracket | OpenSquareBracket
            | ClosingSquareBracket | OpenBracket | ClosingBracket | Colon | DoubleColon
            | Period | DotDot | At | Hash | Arrow | ThickArrow => "terminator",

            While | Sync => {
                "reserved keyword (not currently in use, but may be used in future updates)"
            }

            Return | Implement | For | In | ErrorStatement | If | Else | Object | Trigger
            | Import | Extract | Null | Type | Let | SelfVal | Break | Continue | Switch | Case => {
                "keyword"
            }
            //Comment | MultiCommentStart | MultiCommentEnd => "comment",
            StatementSeparator => "statement separator",
            Error => "unknown",
        }
    }
}

pub struct ParseNotes {
    pub tag: ast::Attribute,
    pub file: PathBuf,
}

impl ParseNotes {
    pub fn new(path: PathBuf) -> Self {
        ParseNotes {
            tag: ast::Attribute::new(),
            file: path,
        }
    }
}

#[derive(Clone)]
pub struct Tokens<'a> {
    iter: Lexer<'a, Token>,
    stack: Vec<(Option<Token>, String, core::ops::Range<usize>)>,
    line_breaks: Vec<u32>,
    //index 0 = element of iter / last element in stack
    index: usize,
}

impl<'a> Tokens<'a> {
    fn new(iter: Lexer<'a, Token>) -> Self {
        Tokens {
            iter,
            stack: Vec::new(),
            line_breaks: vec![0],
            index: 0,
        }
    }

    fn inner_next(&mut self) -> Option<Token> {
        if self.index == 0 {
            let next_elem = self.iter.next();

            let slice = self.iter.slice().to_string();
            let range = self.iter.span();

            self.stack.push((next_elem, slice, range));
            next_elem
        } else {
            self.index -= 1;
            self.stack[self.stack.len() - self.index - 1].0
        }
    }

    fn next(&mut self, ss: bool) -> Option<Token> {
        //println!("what ok {}", self.index);

        // if self.index > 0 {
        //     let curr_element = self.stack[self.stack.len() - self.index].0;

        //     // if curr_element == Some(Token::MultiCommentStart) {
        //     //     //println!("comment time");
        //     //     let mut nest = 0;
        //     //     loop {
        //     //         let next_elem = self.inner_next();

        //     //         if next_elem == Some(Token::MultiCommentStart) {
        //     //             nest += 1;
        //     //         } else if next_elem == Some(Token::MultiCommentEnd) {
        //     //             nest -= 1;
        //     //         }

        //     //         if nest == 0 {
        //     //             break;
        //     //         }
        //     //     }

        //     //     self.inner_next();
        //     // }
        // }

        let next_element = self.inner_next();

        if !ss && next_element == Some(Token::StatementSeparator) {
            self.next(ss)
        } else {
            next_element
        }
    }

    fn previous(&mut self) -> Option<Token> {
        /*self.index += 1;
        let len = self.stack.len();
        if len > self.index {
            if self.stack[len - self.index - 1].0 == Token::StatementSeparator
                || self.stack[len - self.index - 1].0 == Token::Comment
            {
                self.previous()
            } else if len - self.index >= 1 {
                Some(self.stack[len - self.index - 1].0)
            } else {
                None
            }
        } else {
            None
        }*/
        self.previous_no_ignore(false)
    }

    fn previous_no_ignore(&mut self, ss: bool) -> Option<Token> {
        self.index += 1;
        let len = self.stack.len();
        if len > self.index {
            if !ss && self.stack[len - self.index - 1].0 == Some(Token::StatementSeparator) {
                self.previous_no_ignore(ss)
            } else if len - self.index >= 1 {
                self.stack[len - self.index - 1].0
            } else {
                None
            }
        } else {
            None
        }
    }

    /*fn current(&self) -> Option<Token> {
        let len = self.stack.len();
        if len == 0 {
            None
        } else if len - self.index < 1 {
            None
        } else {
            Some(self.stack[len - self.index - 1].0)
        }
    }*/

    fn slice(&self) -> String {
        self.stack[self.stack.len() - self.index - 1].1.clone()
    }

    fn position(&self) -> (usize, usize) {
        if self.stack.len() - self.index == 0 {
            return (0, 0);
        }
        let file_pos1 = self.stack[self.stack.len() - self.index - 1].2.start;
        let file_pos2 = self.stack[self.stack.len() - self.index - 1].2.end;
        (file_pos1, file_pos2)
    }

    /*fn abs_position(&self) -> usize {
        self.stack[self.stack.len() - self.index - 1].2.start
    }*/

    /*fn span(&self) -> core::ops::Range<usize> {
        self.stack[self.stack.len() - self.index - 1].2.clone()
    }*/
}

//type TokenList = Peekable<Lexer<Token>>;

const STATEMENT_SEPARATOR_DESC: &str = "Statement separator (line-break or ';')";

pub fn parse_spwn(
    mut unparsed: String,
    path: PathBuf,
) -> Result<(Vec<ast::Statement>, ParseNotes), SyntaxError> {
    unparsed = unparsed.replace("\r\n", "\n");

    let tokens_iter = Token::lexer(&unparsed);

    let mut tokens = Tokens::new(tokens_iter);

    let mut statements = Vec::<ast::Statement>::new();

    let mut notes = ParseNotes::new(path);

    let mut line_breaks = Vec::<u32>::new();
    let mut current_index: u32 = 0;

    for line in unparsed.lines() {
        current_index += line.len() as u32;
        line_breaks.push(current_index);
        current_index += 1; //line break char
    }

    tokens.line_breaks = line_breaks;

    let start_tag = check_for_tag(&mut tokens, &mut notes)?;
    notes.tag = start_tag;
    loop {
        //+ do something if we have tokens. if no more tokens, leave loop
        match tokens.next(false) {
            //oops we just advanced the tokens in an attempt to check if we have any
            Some(_) => {
                tokens.previous_no_ignore(false); //bring tokens back to original

                //+ we are going to parse the tokens
                let parsed = parse_statement(&mut tokens, &mut notes)?;
                // if parsed.comment.0 == None && !statements.is_empty() {
                //     parsed.comment.0 = statements.last().unwrap().comment.1.clone();
                //     (*statements.last_mut().unwrap()).comment.1 = None;
                // }

                statements.push(parsed)
            }
            None => break, //+ no more tokens, probably end of file
        }

        //+ can't find any more tokens that are valid syntax, checking for line separator
        match tokens.next(true) {
            Some(Token::StatementSeparator) => {}
            Some(a) => {
                return Err(SyntaxError::ExpectedErr {
                    expected: STATEMENT_SEPARATOR_DESC.to_string(),
                    found: format!("{}: \"{}\"", a.typ(), tokens.slice()),
                    pos: tokens.position(),
                    file: notes.file,
                })
            }
            None => break,
        }
    }

    Ok((statements, notes))
}

fn parse_cmp_stmt(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<Vec<ast::Statement>, SyntaxError> {
    let mut statements = Vec::<ast::Statement>::new();
    let opening_bracket = tokens.position();
    loop {
        match tokens.next(false) {
            Some(Token::ClosingCurlyBracket) => break,
            Some(_) => {
                tokens.previous_no_ignore(false);

                let parsed = parse_statement(tokens, notes)?;
                // if parsed.comment.0 == None && !statements.is_empty() {
                //     parsed.comment.0 = statements.last().unwrap().comment.1.clone();
                //     (*statements.last_mut().unwrap()).comment.1 = None;
                // }

                statements.push(parsed) // add to big statement list
                                        //println!("statement done");
            }
            None => {
                return Err(SyntaxError::SyntaxError {
                    message: "Couldn't find matching '}' for this '{'".to_string(),
                    pos: opening_bracket,
                    file: notes.file.clone(),
                })
            }
        }

        match tokens.next(true) {
            Some(Token::StatementSeparator) => {}
            Some(Token::ClosingCurlyBracket) => break,
            a => expected!(STATEMENT_SEPARATOR_DESC.to_string(), tokens, notes, a),
        }
    }
    //tokens.next(false, false);
    Ok(statements)
}

pub fn parse_statement(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<ast::Statement, SyntaxError> {
    //let preceding_comment = check_for_comment(tokens);

    //let mut comment_after = None;

    let first = tokens.next(false);

    let (start_pos, _) = tokens.position();

    let mut arrow = false;
    let body = match first {
        // ooh what type of token is it
        Some(Token::Arrow) => {
            //parse async statement
            if tokens.next(false) == Some(Token::Arrow) {
                //double arrow (throw error)
                return Err(SyntaxError::UnexpectedErr {
                    found: "double arrow (-> ->)".to_string(),
                    pos: tokens.position(),
                    file: notes.file.clone(),
                });
            }

            tokens.previous();

            let rest_of_statement = parse_statement(tokens, notes)?; // recursion moment

            arrow = true;
            rest_of_statement.body

            /* Summary:
            check for double arrows, if it is then throw error because you cant do that
            enable the async flag and parse everything else
            */
        }

        Some(Token::Return) => {
            //parse return statement

            match tokens.next(true) {
                //do we actually return something?
                Some(Token::StatementSeparator) | Some(Token::ClosingCurlyBracket) => {
                    // we dont return anything
                    tokens.previous();
                    ast::StatementBody::Return(None)
                }

                _ => {
                    // we are returning something, how fun
                    tokens.previous();
                    let expr = parse_expr(tokens, notes, true, true)?; //parse whatever we are returning
                                                                       // comment_after =
                                                                       //     if let Some(comment) = expr.values.last().unwrap().comment.1.clone() {
                                                                       //         (*expr.values.last_mut().unwrap()).comment.1 = None;
                                                                       //         Some(comment)
                                                                       //     } else {
                                                                       //         None
                                                                       //     };
                    ast::StatementBody::Return(Some(expr))
                }
            }

            /* Summary:
                check if we are returning something
                if not, just output an empty return syntax tree
                if we are, parse the return expression and return a syntax tree with it
            */
        }

        Some(Token::Break) => ast::StatementBody::Break, // its just break
        Some(Token::Continue) => ast::StatementBody::Continue,

        Some(Token::If) => {
            //parse if statement

            // println!("if statement");

            let condition = parse_expr(tokens, notes, true, false)?; // parse the condition part
            match tokens.next(false) {
                // check for a { character
                Some(Token::OpenCurlyBracket) => (),
                a => {
                    // no { this is very bad
                    expected!("'{'".to_string(), tokens, notes, a)
                }
            }
            let if_body = parse_cmp_stmt(tokens, notes)?; // parse whatever is inside if statement

            let else_body = match tokens.next(false) {
                // is there an else?
                Some(Token::Else) => match tokens.next(false) {
                    // there is an else, check for else if
                    Some(Token::OpenCurlyBracket) => {
                        // no else if, just else
                        Some(parse_cmp_stmt(tokens, notes)?) // parse the else
                    }
                    Some(Token::If) => {
                        // there is an else if
                        tokens.previous();

                        Some(vec![parse_statement(tokens, notes)?]) // parse the else if
                    }

                    a => {
                        // found something other than { or if
                        expected!("'{' or 'if'".to_string(), tokens, notes, a)
                    }
                },

                _ => {
                    // no else at all
                    tokens.previous();
                    None
                }
            };

            let if_statement = ast::If {
                condition,
                if_body,
                else_body,
            };

            ast::StatementBody::If(if_statement)

            /* Summary:
                parse the condition
                check if the first "if" statement has a {
                check for any "else" or "else if" statements
                return a syntax tree for it
            */
        }

        Some(Token::For) => {
            //parse for statement

            let symbol = match tokens.next(false) {
                // check for variable
                Some(Token::Symbol) => tokens.slice(),

                Some(a) => {
                    // invalid variable name
                    return Err(SyntaxError::ExpectedErr {
                        expected: "iterator variable name".to_string(),
                        found: format!("{}: \"{}\"", a.typ(), tokens.slice()),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }

                None => {
                    // literally no variable, why
                    return Err(SyntaxError::ExpectedErr {
                        expected: "iterator variable name".to_string(),
                        found: "None".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }
            };

            match tokens.next(false) {
                // check for an in
                Some(Token::In) => {}
                a => {
                    // didnt find an in
                    expected!("keyword 'in'".to_string(), tokens, notes, a)
                }
            };

            let array = parse_expr(tokens, notes, true, false)?; // parse the array (or range)
            match tokens.next(false) {
                // check for brace
                Some(Token::OpenCurlyBracket) => {}
                a => {
                    // no brace
                    expected!("'{'".to_string(), tokens, notes, a)
                }
            };
            let body = parse_cmp_stmt(tokens, notes)?; // parse whats in the for loop

            ast::StatementBody::For(ast::For {
                symbol: Intern::new(symbol),
                array,
                body,
            })
            /* Summary:
            check for an iterator variable
            parse range/list
            parse code block
            return syntax tree
            */
        }

        Some(Token::ErrorStatement) => {
            let expr = parse_expr(tokens, notes, true, true)?;
            // comment_after = if let Some(comment) = expr.values.last().unwrap().comment.1.clone() {
            //     (*expr.values.last_mut().unwrap()).comment.1 = None;
            //     Some(comment)
            // } else {
            //     None
            // };
            ast::StatementBody::Error(ast::Error { message: expr })
            //i dont think a summary is needed for this
        }

        Some(Token::Type) => {
            // defining a new type
            match tokens.next(false) {
                // all types start with @, throw error if it doesn't
                Some(Token::At) => (),
                a => expected!("'@'".to_string(), tokens, notes, a),
            };

            match tokens.next(false) {
                // check if type name is valid
                Some(Token::Symbol) => ast::StatementBody::TypeDef(tokens.slice()),
                a => expected!("type name".to_string(), tokens, notes, a),
            }
            /*Summary:
            check for @ symbol at the start
            check if type name is actually valid
            return typedef syntax tree
            */
        }

        Some(Token::Implement) => {
            //parse impl statement
            let symbol = parse_variable(tokens, notes, true)?;
            /*
                You might be asking yourself here,
                "why are we parsing it as a variable and not a type?"

                Well the answer to that is simply that the developer thought
                that some people might not like the typing system and would
                want to use a variable instead.
            */

            match tokens.next(false) {
                // check if it has the brace
                Some(Token::OpenCurlyBracket) => ast::StatementBody::Impl(ast::Implementation {
                    symbol,
                    members: parse_dict(tokens, notes)?, // impl block is basically a dict
                }),

                a => {
                    // no brace
                    expected!("'{'".to_string(), tokens, notes, a)
                }
            }
            // honestly this shouldn't deserve a summary its so basic
        }

        Some(Token::Extract) => {
            let expr = parse_expr(tokens, notes, true, true)?;
            // its an expression because dicts can also be extracted alongside imported modules

            ast::StatementBody::Extract(expr)
            // too basic to have a summary,
        }

        Some(_) => {
            //either expression, call or definition, FIGURE OUT
            //parse it

            //expression or call
            tokens.previous_no_ignore(false);
            let expr = parse_expr(tokens, notes, true, true)?;
            if tokens.next(false) == Some(Token::Exclamation) {
                //call
                ast::StatementBody::Call(ast::Call {
                    function: expr.values[0].clone(),
                })
            } else {
                // expression statement
                // println!("found expr");
                tokens.previous_no_ignore(false);

                // comment_after = if let Some(comment) = expr.values.last().unwrap().comment.1.clone()
                // {
                //     (*expr.values.last_mut().unwrap()).comment.1 = None;
                //     Some(comment)
                // } else {
                //     None
                // };
                /*println!(
                    "current token after stmt post comment: {}: ",
                    tokens.slice()
                );*/

                ast::StatementBody::Expr(expr)
            }
        }

        None => {
            //end of input
            unimplemented!()
        }
    };
    let (_, end_pos) = tokens.position();
    // if comment_after == None {
    //     comment_after = check_for_comment(tokens);
    // }
    /*println!(
        "current token after stmt post comment: {}: ",
        tokens.slice()
    );*/

    Ok(ast::Statement {
        // we are returning a statement pog
        body,
        arrow,
        pos: (start_pos, end_pos),
        // comment: (preceding_comment, comment_after),
    })
}

fn operator_precedence(op: &ast::Operator) -> u8 {
    use ast::Operator::*;
    match op {
        As => 10,
        Power => 9,

        Either => 8,

        Modulo => 7,
        Star => 7,
        Slash => 7,
        IntDividedBy => 7,

        Plus => 6,
        Minus => 6,

        Range => 5,

        MoreOrEqual => 4,
        LessOrEqual => 4,
        More => 3,
        Less => 3,

        Equal => 2,
        Has => 2,
        NotEqual => 2,

        Or => 1,
        And => 1,

        Assign => 0,
        Add => 0,
        Subtract => 0,
        Multiply => 0,
        Divide => 0,
        IntDivide => 0,
        Exponate => 0,
        Modulate => 0,
        Swap => 0,
    }
}

fn fix_precedence(mut expr: ast::Expression) -> ast::Expression {
    for val in &mut expr.values {
        let body = &mut val.value.body;
        if let ast::ValueBody::Expression(e) = body {
            *e = fix_precedence(e.clone());
        }
    }

    if expr.operators.len() <= 1 {
        expr
    } else {
        let mut lowest = 10;

        for op in &expr.operators {
            let p = operator_precedence(op);
            if p < lowest {
                lowest = p
            };
        }

        let mut new_expr = ast::Expression {
            operators: Vec::new(),
            values: Vec::new(),
        };

        for (i, op) in expr.operators.iter().enumerate().rev() {
            if operator_precedence(op) == lowest {
                new_expr.operators.push(*op);
                new_expr.values.push(if i == expr.operators.len() - 1 {
                    expr.values.last().unwrap().clone()
                } else {
                    // expr.operators[(i + 1)..].to_vec(),
                    //     values: expr.values[(i + 1)..]
                    fix_precedence(ast::Expression {
                        operators: expr.operators[(i + 1)..].to_vec(),
                        values: expr.values[(i + 1)..].to_vec(),
                    })
                    .to_variable()
                });
                new_expr.values.push(if i == 0 {
                    expr.values[0].clone()
                } else {
                    fix_precedence(ast::Expression {
                        operators: expr.operators[..i].to_vec(),
                        values: expr.values[..(i + 1)].to_vec(),
                    })
                    .to_variable()
                });

                break;
            }
        }
        new_expr.operators.reverse();
        new_expr.values.reverse();

        new_expr
    }
}

fn parse_cases(tokens: &mut Tokens, notes: &mut ParseNotes) -> Result<Vec<ast::Case>, SyntaxError> {
    let mut default_enabled = false;

    //let mut do_we_have_next = true;

    let mut cases = Vec::<ast::Case>::new();
    loop {
        match tokens.next(false) {
            Some(Token::ClosingCurlyBracket) => break,
            Some(Token::Else) => {
                // the default
                if default_enabled {
                    return Err(SyntaxError::SyntaxError {
                        message: "Cannot have 2 else cases".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }
                default_enabled = true;

                /* under normal circumstances we would add another value to check_types,
                but since an else case never type checks and is always at the end,
                there is no need to. */

                match tokens.next(false) {
                    Some(Token::Colon) => {
                        let expr = parse_expr(tokens, notes, false, true)?; // parse whats after the :
                        cases.push(ast::Case {
                            typ: ast::CaseType::Default,
                            body: expr,
                        });
                        if tokens.next(false) != Some(Token::Comma) {
                            // for error formatting
                            tokens.previous();
                        }
                    }
                    a => expected!("':'".to_string(), tokens, notes, a),
                }
            }
            Some(Token::Case) => {
                if default_enabled {
                    return Err(SyntaxError::SyntaxError {
                        message: "cannot have more cases after 'else' field".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }
                let val = parse_expr(tokens, notes, false, true)?;
                match tokens.next(false) {
                    Some(Token::Colon) => {
                        let expr = parse_expr(tokens, notes, false, true)?; // parse whats after the :
                        cases.push(ast::Case {
                            typ: ast::CaseType::Value(val),
                            body: expr,
                        });

                        if tokens.next(false) != Some(Token::Comma) {
                            // for error formatting
                            tokens.previous_no_ignore(false);
                        }
                    }
                    a => expected!("':'".to_string(), tokens, notes, a),
                }
            }

            _ => {
                tokens.previous();

                let pat = parse_expr(tokens, notes, false, true)?;
                if default_enabled {
                    return Err(SyntaxError::SyntaxError {
                        message: "cannot have more cases after 'else' field".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }
                match tokens.next(false) {
                    Some(Token::Colon) => {
                        let expr = parse_expr(tokens, notes, false, true)?; // parse whats after the :
                        cases.push(ast::Case {
                            typ: ast::CaseType::Pattern(pat),
                            body: expr,
                        });

                        if tokens.next(false) != Some(Token::Comma) {
                            // for error formatting
                            tokens.previous_no_ignore(false);
                        }
                    }
                    a => expected!("':'".to_string(), tokens, notes, a),
                }
            }
        }
    }

    Ok(cases)
}

fn parse_expr(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
    allow_mut_op: bool,
    allow_macro_def: bool,
) -> Result<ast::Expression, SyntaxError> {
    // Alright lets parse an expression
    // NOTE: this parses whatever is *after* the current token

    let mut values = Vec::<ast::Variable>::new();
    let mut operators = Vec::<ast::Operator>::new();

    tokens.next(false);
    let (start_pos, _) = tokens.position();
    tokens.previous_no_ignore(false);

    values.push(parse_variable(tokens, notes, allow_macro_def)?);
    // all expressions begin with a variable

    while let Some(t) = tokens.next(false) {
        // keep looking for operators and values

        if let Some(o) = parse_operator(&t) {
            // check if new operator
            let op = if allow_mut_op {
                o
            } else {
                match o {
                    ast::Operator::Assign
                    | ast::Operator::Add
                    | ast::Operator::Subtract
                    | ast::Operator::Multiply
                    | ast::Operator::Divide => break,
                    _ => o,
                }
            };

            operators.push(op);
            values.push(parse_variable(tokens, notes, allow_macro_def)?);
        } else {
            break;
        }
    }

    tokens.previous_no_ignore(false);
    let express = fix_precedence(ast::Expression { values, operators }); //pemdas and stuff

    match tokens.next(true) {
        Some(Token::If) => {
            // oooh ternaries

            // remove any = from the ternary and place into a separate stack
            let mut old_values = express.values.clone();
            let mut old_operators = express.operators;

            let mut tern_values = Vec::<ast::Variable>::new();
            let mut tern_operators = Vec::<ast::Operator>::new();

            match old_values.pop() {
                Some(v) => tern_values.push(v),
                _ => {
                    return Err(SyntaxError::SyntaxError {
                        message: "expected expression before 'if'".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    })
                }
            };

            // iterate though the operators until we get one like =
            while !old_operators.is_empty() {
                if operator_precedence(old_operators.last().unwrap()) == 0 {
                    break;
                }

                match (old_values.pop(), old_operators.pop()) {
                    // pop off of the original expression and put onto ternary stack
                    (Some(v), Some(o)) => {
                        tern_values.push(v);
                        tern_operators.push(o);
                    }
                    (_, _) => unreachable!(),
                }
            }

            let conditional = parse_expr(tokens, notes, false, allow_macro_def)?;

            let do_else = match tokens.next(false) {
                // every ternary needs an else
                Some(Token::Else) => parse_expr(tokens, notes, false, allow_macro_def),
                _ => {
                    return Err(SyntaxError::ExpectedErr {
                        expected: "else".to_string(),
                        found: tokens.slice(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    })
                }
            }?;

            let (end_pos, _) = tokens.position();

            // SPWN syntax structures can get pretty messy with variables, valuebodies,
            // valueliterals, expressions, etc.
            let tern = ast::Ternary {
                condition: conditional,
                if_expr: ast::Expression {
                    values: tern_values,
                    operators: tern_operators,
                },
                else_expr: do_else,
            };

            let ternval_literal = ast::ValueLiteral {
                body: ast::ValueBody::Ternary(tern),
            };

            //println!("operators: {:?} values: {:?}", old_operators, old_values);

            old_values.push(ast::Variable {
                operator: None,
                value: ternval_literal,
                path: vec![],
                pos: (start_pos, end_pos),
                //comment: (None, None),
                tag: ast::Attribute::new(),
            });

            Ok(ast::Expression {
                values: old_values,
                operators: old_operators,
            })
        }
        _ => {
            tokens.previous_no_ignore(false);
            Ok(express)
        }
    }
}

fn parse_operator(token: &Token) -> Option<ast::Operator> {
    // its just a giant match statement
    match token {
        Token::DotDot => Some(ast::Operator::Range),
        Token::Or => Some(ast::Operator::Or),
        Token::And => Some(ast::Operator::And),
        Token::Equal => Some(ast::Operator::Equal),
        Token::NotEqual => Some(ast::Operator::NotEqual),
        Token::MoreOrEqual => Some(ast::Operator::MoreOrEqual),
        Token::LessOrEqual => Some(ast::Operator::LessOrEqual),
        Token::LessThan => Some(ast::Operator::Less),
        Token::MoreThan => Some(ast::Operator::More),
        Token::Star => Some(ast::Operator::Star),
        Token::Power | Token::DoubleStar => Some(ast::Operator::Power),
        Token::Plus => Some(ast::Operator::Plus),
        Token::Minus => Some(ast::Operator::Minus),
        Token::Slash => Some(ast::Operator::Slash),
        Token::IntDividedBy => Some(ast::Operator::IntDividedBy),
        Token::Modulo => Some(ast::Operator::Modulo),
        Token::Either => Some(ast::Operator::Either),

        Token::Assign => Some(ast::Operator::Assign),
        Token::Add => Some(ast::Operator::Add),
        Token::Subtract => Some(ast::Operator::Subtract),
        Token::Multiply => Some(ast::Operator::Multiply),
        Token::Divide => Some(ast::Operator::Divide),
        Token::IntDivide => Some(ast::Operator::IntDivide),
        Token::Exponate => Some(ast::Operator::Exponate),
        Token::Modulate => Some(ast::Operator::Modulate),
        Token::Swap => Some(ast::Operator::Swap),
        Token::Has => Some(ast::Operator::Has),
        Token::As => Some(ast::Operator::As),
        _ => None,
    }
}

fn parse_dict(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<Vec<ast::DictDef>, SyntaxError> {
    let mut defs = Vec::<ast::DictDef>::new();

    loop {
        match tokens.next(false) {
            Some(Token::Symbol) | Some(Token::Type) => {
                let symbol = tokens.slice();

                is_valid_symbol(&symbol, tokens, notes)?;

                let symbol = Intern::new(symbol);

                match tokens.next(false) {
                    Some(Token::Colon) => {
                        let expr = parse_expr(tokens, notes, true, true)?;
                        defs.push(ast::DictDef::Def((symbol, expr)));
                    }
                    Some(Token::Comma) => {
                        if symbol.as_ref() == "type" {
                            return Err(SyntaxError::ExpectedErr {
                                expected: "':'".to_string(),
                                found: String::from("comma (',')"),
                                pos: tokens.position(),
                                file: notes.file.clone(),
                            });
                        }
                        tokens.previous();
                        defs.push(ast::DictDef::Def((
                            symbol,
                            ast::ValueBody::Symbol(symbol)
                                .to_variable(tokens.position())
                                .to_expression(),
                        )));
                    }

                    Some(Token::ClosingCurlyBracket) => {
                        if symbol.as_ref() == "type" {
                            return Err(SyntaxError::ExpectedErr {
                                expected: "':'".to_string(),
                                found: String::from("}"),
                                pos: tokens.position(),
                                file: notes.file.clone(),
                            });
                        }
                        defs.push(ast::DictDef::Def((
                            symbol,
                            ast::ValueBody::Symbol(symbol)
                                .to_variable(tokens.position())
                                .to_expression(),
                        )));
                        //tokens.previous();
                        break;
                    }
                    a => expected!("':'".to_string(), tokens, notes, a),
                }
            }

            Some(Token::DotDot) => {
                let expr = parse_expr(tokens, notes, true, true)?;
                defs.push(ast::DictDef::Extract(expr))
            }

            Some(Token::ClosingCurlyBracket) => break,

            a => expected!(
                "member definition, '..' or '}'".to_string(),
                tokens,
                notes,
                a
            ),
        };
        let next = tokens.next(false);

        if next == Some(Token::ClosingCurlyBracket) {
            break;
        }

        if next != Some(Token::Comma) {
            return Err(SyntaxError::ExpectedErr {
                expected: "comma (',')".to_string(),
                found: format!("{:?}: {:?}", next, tokens.slice()),
                pos: tokens.position(),
                file: notes.file.clone(),
            });
        }
    }
    Ok(defs)
}

fn parse_object(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<Vec<(ast::Expression, ast::Expression)>, SyntaxError> {
    let mut defs = Vec::<(ast::Expression, ast::Expression)>::new();

    match tokens.next(false) {
        Some(Token::OpenCurlyBracket) => (),
        a => expected!("'{'".to_string(), tokens, notes, a),
    }

    loop {
        if tokens.next(false) == Some(Token::ClosingCurlyBracket) {
            break;
        } else {
            tokens.previous();
        }
        let key = parse_expr(tokens, notes, true, true)?;
        match tokens.next(false) {
            Some(Token::Colon) => (),
            a => expected!("':'".to_string(), tokens, notes, a),
        }
        let val = parse_expr(tokens, notes, true, true)?;

        defs.push((key, val));

        let next = tokens.next(false);

        if next == Some(Token::ClosingCurlyBracket) {
            break;
        }

        if next != Some(Token::Comma) {
            return Err(SyntaxError::ExpectedErr {
                expected: "comma (',')".to_string(),
                found: format!("{:?}: {:?}", next, tokens.slice()),
                pos: tokens.position(),
                file: notes.file.clone(),
            });
        }
    }
    Ok(defs)
}

fn parse_args(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<Vec<ast::Argument>, SyntaxError> {
    let mut args = Vec::<ast::Argument>::new();
    let opening_bracket = tokens.position();
    loop {
        if tokens.next(false) == Some(Token::ClosingBracket) {
            break;
        };

        args.push(match tokens.next(false) {
            Some(Token::Assign) => {
                // println!("assign ");
                match tokens.previous() {
                    Some(Token::Symbol) => (),
                    Some(a) => {
                        return Err(SyntaxError::ExpectedErr {
                            expected: "Argument name".to_string(),
                            found: format!("{}: \"{}\"", a.typ(), tokens.slice()),
                            pos: tokens.position(),
                            file: notes.file.clone(),
                        })
                    }

                    None => unreachable!(),
                };
                let start = tokens.position().0;
                let symbol = Some(Intern::new(tokens.slice()));
                tokens.next(false);
                let value = parse_expr(tokens, notes, true, true)?;
                let end = tokens.position().1;
                //tokens.previous();

                ast::Argument {
                    symbol,
                    value,
                    pos: (start, end),
                }
            }

            Some(_) => {
                tokens.previous();
                tokens.previous();
                // println!("arg with no val");

                let value = parse_expr(tokens, notes, true, true)?;

                ast::Argument {
                    symbol: None,
                    pos: value.get_pos(),
                    value,
                }
            }
            None => {
                return Err(SyntaxError::SyntaxError {
                    message: "Couldn't find matching ')' for this '('".to_string(),
                    pos: opening_bracket,
                    file: notes.file.clone(),
                })
            }
        });

        match tokens.next(false) {
            Some(Token::Comma) => (),
            Some(Token::ClosingBracket) => {
                break;
            }

            Some(a) => {
                return Err(SyntaxError::ExpectedErr {
                    expected: "comma (',') or ')'".to_string(),
                    found: format!("{}: \"{}\"", a.typ(), tokens.slice()),
                    pos: tokens.position(),
                    file: notes.file.clone(),
                })
            }

            None => {
                return Err(SyntaxError::SyntaxError {
                    message: "Couldn't find matching ')' for this '('".to_string(),
                    pos: opening_bracket,
                    file: notes.file.clone(),
                })
            }
        }
    }
    //tokens.previous();

    Ok(args)
}

fn parse_arg_def(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<Vec<ast::ArgDef>, SyntaxError> {
    let mut args = Vec::<ast::ArgDef>::new();
    let opening_bracket = tokens.position();
    loop {
        let properties = check_for_tag(tokens, notes)?;
        if tokens.next(false) == Some(Token::ClosingBracket) {
            break;
        };
        args.push(match tokens.next(false) {
            Some(Token::Assign) => {
                if tokens.previous() == Some(Token::SelfVal) {
                    return Err(SyntaxError::SyntaxError {
                        message: "\"self\" argument cannot have a default value".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }
                let start = tokens.position().0;
                let symbol = Intern::new(tokens.slice());
                tokens.next(false);
                let value = Some(parse_expr(tokens, notes, true, true)?);
                let end = tokens.position().1;
                //tokens.previous();

                (symbol, value, properties, None, (start, end))
            }

            Some(Token::Colon) => {
                if tokens.previous() == Some(Token::SelfVal) {
                    return Err(SyntaxError::SyntaxError {
                        message: "\"self\" argument cannot have explicit type".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }
                let start = tokens.position().0;
                let symbol = Intern::new(tokens.slice());
                tokens.next(false);
                let type_value = Some(parse_expr(tokens, notes, false, true)?);
                //tokens.previous();

                match tokens.next(false) {
                    Some(Token::Assign) => {
                        let value = Some(parse_expr(tokens, notes, true, true)?);
                        let end = tokens.position().1;
                        //tokens.previous();

                        (symbol, value, properties, type_value, (start, end))
                    }
                    Some(_) => {
                        tokens.previous();
                        let end = tokens.position().1;
                        (symbol, None, properties, type_value, (start, end))
                    }
                    None => {
                        return Err(SyntaxError::SyntaxError {
                            message: "Couldn't find matching ')' for this '('".to_string(),
                            pos: opening_bracket,
                            file: notes.file.clone(),
                        })
                    }
                }
            }

            Some(_) => {
                if tokens.previous() == Some(Token::SelfVal) && !args.is_empty() {
                    return Err(SyntaxError::SyntaxError {
                        message: "\"self\" argument must be the first argument".to_string(),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    });
                }

                (
                    Intern::new(tokens.slice()),
                    None,
                    properties,
                    None,
                    tokens.position(),
                )
            }
            None => {
                return Err(SyntaxError::SyntaxError {
                    message: "Couldn't find matching ')' for this '('".to_string(),
                    pos: opening_bracket,
                    file: notes.file.clone(),
                })
            }
        });

        match tokens.next(false) {
            Some(Token::Comma) => (),
            Some(Token::ClosingBracket) => break,

            Some(a) => {
                return Err(SyntaxError::ExpectedErr {
                    expected: "comma (',') or ')'".to_string(),
                    found: format!("{}: \"{}\"", a.typ(), tokens.slice()),
                    pos: tokens.position(),
                    file: notes.file.clone(),
                })
            }

            None => {
                return Err(SyntaxError::SyntaxError {
                    message: "Couldn't find matching ')' for this '('".to_string(),
                    pos: opening_bracket,
                    file: notes.file.clone(),
                })
            }
        }
    }
    //tokens.previous();

    Ok(args)
}

fn check_for_tag(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
) -> Result<ast::Attribute, SyntaxError> {
    let first = tokens.next(false);

    match first {
        Some(Token::Hash) => {
            //parse tag
            match tokens.next(false) {
                Some(Token::OpenSquareBracket) => (),
                a => expected!("'['".to_string(), tokens, notes, a),
            };

            let mut contents = ast::Attribute::new();

            loop {
                match tokens.next(false) {
                    Some(Token::ClosingSquareBracket) => break,
                    Some(Token::Symbol) => {
                        let name = tokens.slice();
                        let args = match tokens.next(false) {
                            Some(Token::OpenBracket) => parse_args(tokens, notes)?,
                            Some(Token::Comma) => Vec::new(),
                            Some(Token::ClosingSquareBracket) => {
                                contents.tags.push((name, Vec::new()));
                                break;
                            }
                            a => expected!(
                                "either '(', ']' or comma (',')".to_string(),
                                tokens,
                                notes,
                                a
                            ),
                        };
                        contents.tags.push((name, args));
                    }
                    a => expected!("either Symbol or ']'".to_string(), tokens, notes, a),
                };
            }

            Ok(contents)
        }
        _ => {
            tokens.previous_no_ignore(false);
            Ok(ast::Attribute::new())
        }
    }
}

pub fn str_content(
    mut inp: String,
    tokens: &Tokens,
    notes: &ParseNotes,
) -> Result<String, SyntaxError> {
    inp.remove(0);
    inp.remove(inp.len() - 1);
    let mut out = String::new();
    let mut chars = inp.chars();

    while let Some(c) = chars.next() {
        out.push(if c == '\\' {
            match chars.next() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('"') => '\"',
                Some('\'') => '\'',
                Some('\\') => '\\',
                Some(a) => {
                    return Err(SyntaxError::SyntaxError {
                        message: format!("Invalid escape: \\{}", a),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    })
                }
                None => unreachable!(),
            }
        } else {
            c
        });
    }

    Ok(out)
}

fn check_if_slice(mut tokens: Tokens, notes: &mut ParseNotes) -> Result<bool, SyntaxError> {
    loop {
        match tokens.next(false) {
            Some(Token::Colon) => {
                return Ok(true);
            }
            Some(Token::ClosingSquareBracket) => {
                return Ok(false);
            }
            _ => {
                tokens.previous_no_ignore(false);
                parse_expr(&mut tokens, notes, true, true)?;
            }
        };
    }
}

fn parse_macro(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
    properties: ast::Attribute,
    decorator: Option<Box<ast::Variable>>
) -> Result<ast::ValueBody, SyntaxError> {
    let parse_macro_def = |tokens: &mut Tokens,
                           notes: &mut ParseNotes|
     -> Result<ast::ValueBody, SyntaxError> {
        let arg_start = tokens.position().0;
        let args = parse_arg_def(tokens, notes)?;
        let arg_end = tokens.position().1;
        let body = match tokens.next(false) {
            Some(Token::OpenCurlyBracket) => parse_cmp_stmt(tokens, notes)?,
            Some(Token::ThickArrow) => {
                let start = tokens.position().0;
                let expr = parse_expr(tokens, notes, true, true)?;
                let end = tokens.position().1;
                vec![ast::Statement {
                    body: ast::StatementBody::Return(Some(expr)),
                    arrow: false,
                    //comment: (None, None),
                    pos: (start, end),
                }]
            }
            a => expected!("'{'".to_string(), tokens, notes, a),
        };

        let m_value = ast::ValueBody::Macro(ast::Macro {
            args,
            body: ast::CompoundStatement { statements: body },
            properties: properties.clone(),
            arg_pos: (arg_start, arg_end)
        });

        if let Some(d) = decorator {
            let mut unwrapped_deco = (*d).clone();

            let m_var = ast::Variable {
                value: ast::ValueLiteral::new(m_value),
                path: Vec::new(),
                operator: None,
                pos: (arg_start, arg_end),
                tag: properties.clone()
            };

            let mut new_path = vec![ast::Argument {
                symbol: None,
                value: m_var.to_expression(),
                pos: (arg_start, arg_end)
            }];

            if let Some(p_) = unwrapped_deco.path.pop() {
                match p_ {
                    ast::Path::Call(v) => {
                        new_path.extend(v);
                    },
                    _ => {
                        unwrapped_deco.path.push(p_);
                    }
                }
            }

            unwrapped_deco.path.push(ast::Path::Call(new_path));

            Ok(ast::ValueBody::Expression(unwrapped_deco.to_expression()))
        } else {
            Ok(m_value)
        }
    };

    let mut test_tokens = tokens.clone();

    return match parse_expr(&mut test_tokens, notes, true, true) {
        Ok(expr) => {
            //macro def
            Ok(match test_tokens.next(false) {
                Some(Token::ClosingBracket) => match test_tokens.next(false) {
                    Some(Token::OpenCurlyBracket) => parse_macro_def(tokens, notes)?,
                    Some(Token::ThickArrow) => parse_macro_def(tokens, notes)?,
                    _ => {
                        test_tokens.previous();
                        (*tokens) = test_tokens;
                        ast::ValueBody::Expression(expr)
                    }
                },
                Some(Token::Comma) => parse_macro_def(tokens, notes)?,
                Some(Token::Colon) => parse_macro_def(tokens, notes)?,
                a => {
                    return Err(SyntaxError::ExpectedErr {
                        expected: "')', ':' or comma (',')".to_string(),
                        found: format!("{:?}: {:?}", a, test_tokens.slice()),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    })
                }
            })
        }

        Err(_) => match parse_macro_def(tokens, notes) {
            Ok(mac) => Ok(mac),
            Err(e) => return Err(e),
        },
    };
}

fn parse_variable(
    tokens: &mut Tokens,
    notes: &mut ParseNotes,
    allow_macro_def: bool,
    //check_for_comments: bool,
) -> Result<ast::Variable, SyntaxError> {
    // for the vars and stuff that isnt operators
    // let preceding_comment = if check_for_comments {
    //     check_for_comment(tokens)
    // } else {
    //     None
    // };

    let properties = check_for_tag(tokens, notes)?;

    let mut first_token = tokens.next(false);
    let (start_pos, _) = tokens.position();

    let operator = match first_token {
        // does it start with an op? (e.g -3, let i)
        Some(Token::Minus) => {
            first_token = tokens.next(false);
            Some(ast::UnaryOperator::Minus)
        }
        Some(Token::Exclamation) => {
            if tokens.next(true) == Some(Token::OpenCurlyBracket) {
                tokens.previous_no_ignore(true);
                None
            } else {
                tokens.previous_no_ignore(true);
                first_token = tokens.next(false);
                Some(ast::UnaryOperator::Not)
            }
        }

        Some(Token::DotDot) => {
            first_token = tokens.next(false);
            Some(ast::UnaryOperator::Range)
        }

        Some(Token::Let) => {
            first_token = tokens.next(false);
            Some(ast::UnaryOperator::Let)
        }
        Some(Token::Increment) => {
            first_token = tokens.next(false);
            Some(ast::UnaryOperator::Increment)
        }
        Some(Token::Decrement) => {
            first_token = tokens.next(false);
            Some(ast::UnaryOperator::Decrement)
        }
        _ => None,
    };

    let value = match first_token {
        // what kind of variable is it?
        Some(Token::Number) => ast::ValueBody::Number(match tokens.slice().parse() {
            Ok(n) => n, // its a valid number
            Err(err) => {
                return Err(SyntaxError::SyntaxError {
                    message: format!("Error when parsing number: {}", err),

                    pos: tokens.position(),
                    file: notes.file.clone(),
                });
            }
        }),
        Some(Token::StringLiteral) => {
            // is a string
            ast::ValueBody::Str(str_content(tokens.slice(), tokens, notes)?)
        }
        Some(Token::Id) => {
            let mut text = tokens.slice();
            let class_name = match text.pop().unwrap() {
                'g' => ast::IdClass::Group,
                'c' => ast::IdClass::Color,
                'i' => ast::IdClass::Item,
                'b' => ast::IdClass::Block,
                _ => unreachable!(),
            };

            let (unspecified, number) = match text.as_ref() {
                "?" => (true, 0),
                _ => (
                    false,
                    match text.parse() {
                        Ok(n) => n,
                        Err(err) => {
                            return Err(SyntaxError::SyntaxError {
                                message: format!("Error when parsing number: {}", err),

                                pos: tokens.position(),
                                file: notes.file.clone(),
                            });
                        }
                    },
                ),
            };

            ast::ValueBody::Id(ast::Id {
                class_name,
                unspecified,
                number,
            })
        }
        Some(Token::True) => ast::ValueBody::Bool(true),
        Some(Token::False) => ast::ValueBody::Bool(false),
        Some(Token::Null) => ast::ValueBody::Null,
        Some(Token::SelfVal) => ast::ValueBody::SelfVal,
        Some(Token::Symbol) => {
            let symbol = Intern::new(tokens.slice());

            match tokens.next(false) {
                Some(Token::ThickArrow) => {
                    // Woo macro shorthand
                    let arg = if symbol.as_ref() != "_" {
                        is_valid_symbol(&symbol, tokens, notes)?;
                        vec![(symbol, None, properties.clone(), None, tokens.position())]
                    } else {
                        Vec::new()
                    };

                    let start = tokens.position();
                    let expr = parse_expr(tokens, notes, true, true)?;
                    let end = tokens.position().1;
                    let macro_body = vec![ast::Statement {
                        body: ast::StatementBody::Return(Some(expr)),
                        arrow: false,
                        //comment: (None, None),
                        pos: (start.0, end),
                    }];

                    ast::ValueBody::Macro(ast::Macro {
                        args: arg,
                        body: ast::CompoundStatement {
                            statements: macro_body,
                        },
                        arg_pos: start,
                        properties: properties.clone()
                    })
                }
                _ => {
                    is_valid_symbol(&symbol, tokens, notes)?;
                    tokens.previous_no_ignore(false);
                    ast::ValueBody::Symbol(symbol)
                }
            }
        }

        Some(Token::OpenSquareBracket) => {

            let mut potential_macro: Option<ast::ValueBody> = None;

            if let Some(Token::OpenSquareBracket) = tokens.next(false) {
                let mut test_tokens = tokens.clone();
                match parse_variable(&mut test_tokens, notes, false) {
                    Ok(mut v) => {
                        if let Some(Token::ClosingSquareBracket) = test_tokens.next(false) {
                            if let Some(Token::ClosingSquareBracket) = test_tokens.next(false) {
                                match test_tokens.next(false) {
                                    Some(Token::OpenBracket) => {
                                        // its a decorator on a macro
                                        *tokens = test_tokens;
                                        potential_macro = Some(parse_macro(tokens, notes, properties.clone(), Some(Box::new(v)))?);
                                    },
                                    Some(Token::Exclamation) => {
                                        // its a decorator on a trigger function

                                        match test_tokens.next(true) { // fuck this, i ain't allowing !;{ }
                                            Some(Token::OpenCurlyBracket) => (),
                                            a => expected!("{".to_string(), tokens, notes, a)
                                        }

                                        *tokens = test_tokens;

                                        let trig = ast::ValueBody::CmpStmt(ast::CompoundStatement {
                                            statements: parse_cmp_stmt(tokens, notes)?,
                                        });

                                        let t_var = ast::Variable {
                                            value: ast::ValueLiteral::new(trig),
                                            path: Vec::new(),
                                            operator: None,
                                            pos: (0, 0),
                                            tag: properties.clone()
                                        };

                                        let mut new_path = vec![ast::Argument {
                                            symbol: None,
                                            value: t_var.to_expression(),
                                            pos: (0, 0)
                                        }];

                                        if let Some(p_) = v.path.pop() {
                                            match p_ {
                                                ast::Path::Call(vv) => {
                                                    new_path.extend(vv);
                                                },
                                                _ => {
                                                    v.path.push(p_);
                                                }
                                            }
                                        }

                                        v.path.push(ast::Path::Call(new_path));

                                        potential_macro = Some(ast::ValueBody::Expression(v.to_expression()))
                                    },
                                    _ => ()
                                }
                            } 
                        }
                    },
                    Err(_) => ()
                }
            }

            match potential_macro {
                Some(x) => x,
                None => {
                    tokens.previous_no_ignore(false);

                    //Array
                    let mut arr = Vec::new();

                    if tokens.next(false) != Some(Token::ClosingSquareBracket) {
                        tokens.previous();
                        loop {
                            arr.push(parse_expr(tokens, notes, true, true)?);
                            match tokens.next(false) {
                                Some(Token::Comma) => {
                                    //accounting for trailing comma
                                    if let Some(Token::ClosingSquareBracket) = tokens.next(false) {
                                        break;
                                    } else {
                                        tokens.previous();
                                    }
                                }
                                Some(Token::ClosingSquareBracket) => break,
                                a => expected!("comma (',') or ']'".to_string(), tokens, notes, a),
                            }
                        }
                    }

                    ast::ValueBody::Array(arr)
                }
            }
        }

        Some(Token::Import) => {
            let mut first = tokens.next(false);
            let mut forced = false;
            if first == Some(Token::Exclamation) {
                forced = true;
                first = tokens.next(false);
            }
            match first {
                Some(Token::StringLiteral) => ast::ValueBody::Import(
                    ImportType::Script(PathBuf::from(str_content(tokens.slice(), tokens, notes)?)),
                    forced,
                ),
                Some(Token::Symbol) => {
                    ast::ValueBody::Import(ImportType::Lib(tokens.slice()), forced)
                }
                a => expected!("literal string".to_string(), tokens, notes, a),
            }
        }

        Some(Token::At) => {
            let type_name = match tokens.next(false) {
                Some(Token::Symbol) => tokens.slice(),
                a => expected!("type name".to_string(), tokens, notes, a),
            };

            ast::ValueBody::TypeIndicator(type_name)
        }

        Some(Token::Switch) => {
            let value = parse_expr(tokens, notes, true, false)?; // what are we switching?

            let cases = match tokens.next(false) {
                Some(Token::OpenCurlyBracket) => parse_cases(tokens, notes)?, // check for {
                a => expected!("'{'".to_string(), tokens, notes, a),
            };
            //ast::ValueBody::TypeIndicator("number".to_string())
            ast::ValueBody::Switch(value, cases)
        }

        Some(Token::OpenBracket) => {
            if allow_macro_def {
                parse_macro(tokens, notes, properties.clone(), None)?
            } else {
                let expr = parse_expr(tokens, notes, true, true)?;
                match tokens.next(false) {
                    Some(Token::ClosingBracket) => ast::ValueBody::Expression(expr),
                    a => expected!("')'".to_string(), tokens, notes, a),
                }
            }
        }
        Some(Token::OpenCurlyBracket) => ast::ValueBody::Dictionary(parse_dict(tokens, notes)?),
        Some(Token::Exclamation) => {
            //next token is a curly bracket, checked earlier
            tokens.next(false);
            ast::ValueBody::CmpStmt(ast::CompoundStatement {
                statements: parse_cmp_stmt(tokens, notes)?,
            })
        }

        Some(Token::Object) => ast::ValueBody::Obj(ast::ObjectLiteral {
            props: parse_object(tokens, notes)?,
            mode: ast::ObjectMode::Object,
        }),

        Some(Token::Trigger) => ast::ValueBody::Obj(ast::ObjectLiteral {
            props: parse_object(tokens, notes)?,
            mode: ast::ObjectMode::Trigger,
        }),

        a => expected!("a value".to_string(), tokens, notes, a),
    };

    let mut path = Vec::<ast::Path>::new();

    loop {
        match tokens.next(true) {
            Some(Token::OpenSquareBracket) => {
                if check_if_slice(tokens.clone(), notes)? {
                    let mut slices = Vec::<ast::Slice>::new();
                    'main: loop {
                        let mut curr_slice = ast::Slice {
                            left: None,
                            right: None,
                            step: None,
                        };
                        let mut colon_pos = tokens.position();
                        let mut i: i32 = 0;
                        loop {
                            match tokens.next(false) {
                                Some(Token::Colon) => {
                                    colon_pos = tokens.position();
                                    if i == 1 {
                                        curr_slice.step = curr_slice.right.clone();
                                        curr_slice.right = None;
                                    }
                                    i += 1;
                                }

                                Some(Token::ClosingSquareBracket) => {
                                    slices.push(curr_slice);
                                    break 'main;
                                }
                                Some(Token::Comma) => {
                                    slices.push(curr_slice);
                                    continue 'main;
                                }
                                _ => {
                                    tokens.previous_no_ignore(false);
                                    let result = parse_expr(tokens, notes, true, true)?;
                                    match i {
                                        0 => curr_slice.left = Some(result),
                                        1 | 2 => curr_slice.right = Some(result),
                                        _ => {
                                            return Err(SyntaxError::ExpectedErr {
                                                expected: "]".to_string(),
                                                found: ":".to_string(),
                                                pos: colon_pos,
                                                file: notes.file.clone(),
                                            })
                                        }
                                    };
                                }
                            };
                        }
                    }
                    path.push(ast::Path::NSlice(slices));
                } else {
                    let index = parse_expr(tokens, notes, true, true)?;
                    match tokens.next(false) {
                        Some(Token::ClosingSquareBracket) => path.push(ast::Path::Index(index)),

                        a => {
                            return Err(SyntaxError::ExpectedErr {
                                expected: "]".to_string(),
                                found: format!(
                                    "{}: \"{}\"",
                                    match a {
                                        Some(t) => t.typ(),
                                        None => "EOF",
                                    },
                                    tokens.slice()
                                ),
                                pos: tokens.position(),
                                file: notes.file.clone(),
                            })
                        }
                    }
                }
            }
            Some(Token::OpenBracket) => path.push(ast::Path::Call(parse_args(tokens, notes)?)),
            Some(Token::Period) => match tokens.next(false) {
                Some(Token::Symbol) | Some(Token::Type) => {
                    path.push(ast::Path::Member(Intern::new(tokens.slice())))
                }
                a => expected!("member name".to_string(), tokens, notes, a),
            },

            Some(Token::DoubleColon) => match tokens.next(false) {
                Some(Token::Symbol) | Some(Token::Type) => {
                    path.push(ast::Path::Associated(Intern::new(tokens.slice())))
                }
                Some(Token::OpenCurlyBracket) => {
                    path.push(ast::Path::Constructor(parse_dict(tokens, notes)?))
                }
                a => {
                    return Err(SyntaxError::ExpectedErr {
                        expected: "associated member name".to_string(),
                        found: format!(
                            "{}: \"{}\"",
                            match a {
                                Some(t) => t.typ(),
                                None => "EOF",
                            },
                            tokens.slice()
                        ),
                        pos: tokens.position(),
                        file: notes.file.clone(),
                    })
                }
            },

            Some(Token::Increment) => path.push(ast::Path::Increment),
            Some(Token::Decrement) => path.push(ast::Path::Decrement),

            _ => break,
        }
    }
    tokens.previous_no_ignore(false);

    let (_, end_pos) = tokens.position();

    // let comment_after = if check_for_comments {
    //     check_for_comment(tokens)
    // } else {
    //     None
    // };

    /*if tokens.stack.len() - tokens.index > 0 {
        println!("current token after val post comment: {}: ", tokens.slice());
    }*/

    Ok(ast::Variable {
        operator,
        value: ast::ValueLiteral { body: value },
        pos: (start_pos, end_pos),
        //comment: (preceding_comment, comment_after),
        path,
        tag: properties,
    })
}
