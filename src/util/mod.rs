//! Various utility functions and types which do not fit neatly into other 

pub mod number_literal_base;
pub(crate) use number_literal_base::NumberLiteralBase;

use unicode_id_start::{is_id_continue, is_id_start};

/// Checks if a character is a valid first character of an identifier.  
/// Simple wrapper around [`unicode_id_start::is_id_start`()] to include '$' and '_'
pub fn is_identifier_start(c: char) -> bool {
    c == '$' || c == '_' || is_id_start(c)
}

/// Checks if a character is a valid not-first character of an identifier.  
/// Simple wrapper around [`unicode_id_start::is_id_continue`()] to include '$' and '_'
pub fn is_identifier_continue(c: char) -> bool {
    c == '$' || c == '_' || is_id_continue(c)
}
