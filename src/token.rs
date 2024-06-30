#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // grouping
    LParen { offset: usize },
    RParen { offset: usize },
    LBrac { offset: usize },
    RBrac { offset: usize },
    LCurl { offset: usize },
    RCurl { offset: usize },

    // operators
    Plus { offset: usize },
    PlusPlus { offset: usize },
    PlusEqual { offset: usize },
    Minus { offset: usize },
    MinusMinus { offset: usize },
    MinusEqual { offset: usize },
    Slash { offset: usize },
    SlashSlash { offset: usize },
    Modulo { offset: usize },
    Star { offset: usize },
    Hash { offset: usize },
    SemiColon { offset: usize },
    Arrow { offset: usize },
    DoubleArrow { offset: usize },
    Colon { offset: usize },

    // comparisons
    Equal { offset: usize },
    EqualEqual { offset: usize },
    Less { offset: usize },
    LessEqual { offset: usize },
    More { offset: usize },
    MoreEqual { offset: usize },
    Bang { offset: usize },
    BangEqual { offset: usize },

    // literals
    String { offset: usize, value: String },
    Ident { offset: usize, value: String },
    Numeric { offset: usize, value: String },

    // keywords
    Return { offset: usize },
    Func { offset: usize },
    If { offset: usize },
    Else { offset: usize },
    Elif { offset: usize },

    // other
    Eof { offset: usize },
}
