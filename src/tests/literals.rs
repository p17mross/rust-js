//! Tests for value literals and comments

/// Tests that number literals are parsed, and that invalid number literals are rejected
#[test]
fn test_number_literals() {
    // Parsing number literals
    assert_parses!(10);
    assert_parses!(10.0);
    assert_parses!(0x01234567890);
    assert_parses!(0b010);
    assert_parses!(0o012345670);
    assert_parses!(123456789n);
    assert_parses!(10n);
    assert_parses!(0x100n);
    assert_parses!(0b10n);
    assert_parses!(0o0123n);

    assert_not_parses_str!("20.0n"); // BigInt literal with decimal point
    assert_not_parses_str!("10h"); // Number literal with an identifier after
    assert_not_parses_str!("012n"); // Implicit octal BigInt literal
    assert_not_parses_str!("10na"); // BigInt literal with a character after
    assert_not_parses_str!("0xGH"); // Hex literal with wrong characters
    assert_not_parses_str!("0b0123"); // Binary literal with wrong characters
    assert_not_parses_str!("0o123456789"); // Octal literal with wrong characters
}

/// Tests that string literals are parsed, and that invalid string literals are rejected
#[test]
fn test_string_literals() {
    assert_parses_str!(r#" "Some string" "#);
    assert_parses_str!(r#" "Some string with \b \f \n \r \t \v " "#);
    assert_parses_str!(r#" "Single quotes in double quoted string: '' `` " "#);
    assert_parses_str!(r#" 'Double quotes in single quoted string: "" `` ' "#);
    assert_parses_str!(r#" `Double and single quotes in backtick string "" '' ` "#);
    assert_parses_str!(r#" `Newlines in a backtick quoted string
    ` "#);
    assert_parses_str!(r#" "Escaped double quotes: \" " "#);
    assert_parses_str!(r#" 'Escaped single quotes: \' ' "#);
    assert_parses_str!(r#" `Escaped backticks: \` ` "#);
    

    assert_not_parses_str!(r#" "Newline in double quoted string
    " "#);
    assert_not_parses_str!(r#" 'Newline in single quoted string
    ' "#);
    assert_not_parses_str!(r#" "Unclosed double quote string "#);
    assert_not_parses_str!(r#" 'Unclosed single quote string "#);
    assert_not_parses_str!(r#" `Unclosed backtick string "#);
}

/// Tests that 
#[test]
fn test_comments() {
    assert_parses_str!(r#" // A line comment with no newline "#);
    assert_parses_str!(r#" // A line comment with a trailing newline 
    "#);
    assert_parses_str!(r#" // A line comment // This is still commented "#);
    assert_parses_str!(r#" /* A block comment */ "#);
    assert_parses_str!(r#" /* A block comment
    with a newline */ "#);

    assert_not_parses_str!(r#" /* A block comment with no end "#);

}