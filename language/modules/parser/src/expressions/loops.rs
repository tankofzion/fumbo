use crate::primitives::Attributes;

/// Infinite loop expression parse tree structure
pub struct InfiniteLoopExpression {
    /// Attributes (or meta-data) tied to the field
    pub attributes: Attributes,
    pub index: u8,
}

/// Finite loop expression parse tree structure
pub struct ForLoopExpression {
    /// Attributes (or meta-data) tied to the field
    pub attributes: Attributes,
    pub index: u8,
}

/// Predicate loop expression parse tree structure
pub struct WhileLoopExpression {
    /// Attributes (or meta-data) tied to the field
    pub attributes: Attributes,
    pub index: u8,
}
