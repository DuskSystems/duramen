use super::{AstToken, PolicyNode};
use crate::policy::PolicySyntax;

/// A comment token containing text following `//`.
///
/// Comments are preserved in the CST as trivia tokens and can be accessed
/// for formatting and documentation extraction.
///
/// ```cedar
/// // This is a comment
/// permit(principal, action, resource);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CommentToken<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstToken<'a> for CommentToken<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Comment
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl CommentToken<'_> {
    /// Returns the comment content without the `//` prefix.
    ///
    /// Leading whitespace after `//` is preserved.
    ///
    /// ```cedar
    /// // This is a comment
    /// //^ content returns " This is a comment"
    /// ```
    #[must_use]
    pub fn content<'s>(&self, source: &'s str) -> &'s str {
        let text = self.text(source);
        text.strip_prefix("//").unwrap_or(text)
    }

    /// Returns the comment content with leading whitespace trimmed.
    ///
    /// ```cedar
    /// // This is a comment
    /// //^ content_trimmed returns "This is a comment"
    /// ```
    #[must_use]
    pub fn content_trimmed<'s>(&self, source: &'s str) -> &'s str {
        self.content(source).trim_start()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StringToken<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstToken<'a> for StringToken<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::String
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntegerToken<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstToken<'a> for IntegerToken<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Integer
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdentifierToken<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstToken<'a> for IdentifierToken<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Identifier
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}
