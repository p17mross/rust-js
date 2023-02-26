#![cfg(test)]

/// Asserts that a given program parses. The program does not need to be quoted, but this means newlines will be ignored outside of string literals. 
/// For testing code with significant newlines, such as testing semicolon insertion, uss the [`assert_parses_str!`] macro instead.
macro_rules! assert_parses {
    ($($code: tt)*) => {
        assert_parses_str!(stringify!($($code)*))
    };
}

/// Asserts that a given string parses
macro_rules! assert_parses_str {
    ($code: expr) => {{
        use crate::Program;
        Program::from_console($code).expect("Program should have parsed");
    }};
}

/// Asserts that a given program does not parse. The same restrictions surrounding newlines apply as with [`assert_parses!`].
macro_rules! assert_not_parses {
    ($($code: tt)*) => {
        assert_not_parses_str!(stringify!($($code)*))
    };
}

/// Asserts that a given string parses
macro_rules! assert_not_parses_str {
    ($code: expr) => {{
        use crate::Program;
        Program::from_console($code).expect_err("Program should not have parsed");
    }};
}

// Mods need to be after macro definitions or the modules will not compile
mod literals;

/// Asserts that an empty program parses
#[test]
fn test_empty_program() {
    assert_parses!();
}