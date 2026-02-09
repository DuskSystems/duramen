use crate::schema::{ActionDeclaration, EntityDeclaration, TypeDeclaration};

/// A schema declaration.
#[derive(Clone, Debug)]
pub enum Declaration<'a> {
    Entity(EntityDeclaration<'a>),
    Action(ActionDeclaration<'a>),
    Type(TypeDeclaration<'a>),
}
