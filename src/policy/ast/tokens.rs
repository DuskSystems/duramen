use super::{AstToken, PolicyNode, PolicySyntax, ast_token};

ast_token!(CommentToken, PolicySyntax::Comment);

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

ast_token!(StringToken, PolicySyntax::String);
ast_token!(IntegerToken, PolicySyntax::Integer);
ast_token!(IdentifierToken, PolicySyntax::Identifier);
