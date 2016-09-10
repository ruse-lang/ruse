//! Generate a syntax tree from an input stream.

pub mod error;
pub mod stack;
pub mod syntax_tree;

pub use self::error::*;
pub use self::stack::*;
pub use self::syntax_tree::*;

use read::parse;
use read::lex::Token;

/// Parse an input string, returning a syntax tree that can be evaluated.
pub trait Parse {
    /// This function takes in a string representing a ruse expression, and
    /// parses it into an abstract syntax tree.
    ///
    /// The SyntaxTree is a tree of SyntaxNodes, each of which contains the
    /// original text and some data associated with the text.
    ///
    /// For example, the following program:
    ///
    /// ```ruse
    /// (+ 2 3)
    /// ```
    ///
    /// Becomes:
    ///
    /// ```text
    /// [<open_paren>, <ident:+>, <int_literal: 2>, <int_literal: 3>, <close_paren>]
    /// ```
    ///
    /// Becomes:
    ///
    /// ```text
    ///         <fn: '+'>
    ///         /       \
    /// <atom: 2>       <atom: 3>
    /// ```
    ///
    /// The representation is done via a stack.
    ///
    /// For now, this is done by assuming that the first item after an open
    /// paren is a function call, and that everything after is an atom. This
    /// will obviously become better over time.
    fn parse(&self) -> parse::Result {
        unimplemented!()
    }
}

impl Parse for Vec<Token> {}
