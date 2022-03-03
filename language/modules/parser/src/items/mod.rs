mod inputs;
mod datatype;

use syn::Data;

// Re-export items into crate namespace
pub use inputs::ItemInputs;
use crate::items::datatype::ItemDatatype;

/// Item syntax
pub enum Item {

    /// Program inputs item
    Inputs(ItemInputs),

    /// Dependent type item
    Datatype(ItemDatatype)
}