//! Keywords definition
//!
//! This module gathers keywords that are specific to `Woodoo` language.

use syn::custom_keyword;

// Keyword used for declaring a dependently typed [DataType] item.
custom_keyword!(data);

// Keyword used for parsing [ItemInputs] block
custom_keyword!(inputs);

// Keyword used for parsing [ItemOutputs] block
custom_keyword!(outputs);

