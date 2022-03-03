//! Declaration statements
//!
//! A declaration statement introduces one or more names or items into an enclosing statement block.

use crate::items::Item;

/// Declaration statement syntax
pub enum DeclarationStatement {

    /// Item declaration statement
    ItemDeclaration(Item),

    // /// Pattern-based item definition statement
    // LetItemDeclaration(LetItem)
}
