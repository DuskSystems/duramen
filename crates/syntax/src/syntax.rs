use core::fmt;

use duramen_lexer::TokenKind;

use crate::group::Group;
use crate::token::Token;

/// Combined syntax kind stored in the tree.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Syntax {
    /// Token (leaf node).
    Token(Token),
    /// Group (branch node).
    Group(Group),
}

impl Syntax {
    /// Returns the token kind, if this is a token.
    #[must_use]
    pub const fn token(self) -> Option<Token> {
        match self {
            Self::Token(token) => Some(token),
            Self::Group(_) => None,
        }
    }

    /// Returns the group kind, if this is a group.
    #[must_use]
    pub const fn group(self) -> Option<Group> {
        match self {
            Self::Group(group) => Some(group),
            Self::Token(_) => None,
        }
    }

    /// Checks if this is a whitespace token.
    #[must_use]
    pub fn is_whitespace(self) -> bool {
        self.token().is_some_and(Token::is_whitespace)
    }

    /// Checks if this is a newline token.
    #[must_use]
    pub fn is_newline(self) -> bool {
        self.token().is_some_and(Token::is_newline)
    }

    /// Checks if this is a comment token.
    #[must_use]
    pub fn is_comment(self) -> bool {
        self.token().is_some_and(Token::is_comment)
    }

    /// Checks if this is an error (unknown token or error group).
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(
            self,
            Self::Token(Token::Unknown) | Self::Group(Group::Error)
        )
    }

    /// Checks if this is a reserved keyword in policy context.
    #[must_use]
    pub fn is_reserved(self) -> bool {
        self.token().is_some_and(Token::is_reserved)
    }

    /// Checks if this can be used as an identifier.
    #[must_use]
    pub fn is_identifier(self) -> bool {
        self.token().is_some_and(Token::is_identifier)
    }

    /// Checks if this is a keyword token.
    #[must_use]
    pub fn is_keyword(self) -> bool {
        self.token().is_some_and(Token::is_keyword)
    }

    /// Checks if this is a literal token.
    #[must_use]
    pub fn is_literal(self) -> bool {
        self.token().is_some_and(Token::is_literal)
    }

    /// Checks if this is a string token.
    #[must_use]
    pub const fn is_string(self) -> bool {
        matches!(self, Self::Token(Token::String))
    }

    /// Checks if this is a token.
    #[must_use]
    pub const fn is_token(self) -> bool {
        matches!(self, Self::Token(_))
    }

    /// Checks if this is a group.
    #[must_use]
    pub const fn is_group(self) -> bool {
        matches!(self, Self::Group(_))
    }
}

impl fmt::Debug for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token(token) => token.fmt(f),
            Self::Group(group) => group.fmt(f),
        }
    }
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token(token) => token.fmt(f),
            Self::Group(group) => group.fmt(f),
        }
    }
}

impl From<Token> for Syntax {
    fn from(value: Token) -> Self {
        Self::Token(value)
    }
}

impl From<Group> for Syntax {
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl From<TokenKind> for Syntax {
    fn from(value: TokenKind) -> Self {
        Self::Token(Token::from(value))
    }
}

impl PartialEq<Token> for Syntax {
    fn eq(&self, other: &Token) -> bool {
        matches!(self, Self::Token(token) if token == other)
    }
}

impl PartialEq<Syntax> for Token {
    fn eq(&self, other: &Syntax) -> bool {
        other == self
    }
}

impl PartialEq<Group> for Syntax {
    fn eq(&self, other: &Group) -> bool {
        matches!(self, Self::Group(group) if group == other)
    }
}

impl PartialEq<Syntax> for Group {
    fn eq(&self, other: &Syntax) -> bool {
        other == self
    }
}
