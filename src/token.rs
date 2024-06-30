#[derive(Debug, PartialEq)]
pub enum Token {
    // grouping
    LParen { offset: usize },
    RParen { offset: usize },

    // operators
    Plus { offset: usize },
    PlusPlus { offset: usize },
    PlusEqual { offset: usize },

    // comparisons
    Equal { offset: usize },
    EqualEqual { offset: usize },

    // literals
    String { offset: usize, value: String },
    Indent { offset: usize, value: String },
    Number { offset: usize, value: String },

    // keywords
    Return { offset: usize },
    Fn { offset: usize },

    // other
    SemiColon { offset: usize },
    Eof { offset: usize },
}