//! Tests for value literals and comments

use super::*;

/// Tests that number literals are parsed, and that invalid number literals are rejected
#[test]
fn test_number_literals() {
    assert_parses("10");
    assert_parses("10.0");

    assert_parses("0x01234567890");
    assert_parses("0b010");
    assert_parses("0o012345670");

    assert_parses("123456789n");
    assert_parses("10n");
    assert_parses("0x100n");
    assert_parses("0b10n");
    assert_parses("0o0123n");
    
    assert_parses("1_000"); // Underscores as separators
    assert_parses("1_000n");
    assert_parses("1_000.000_001");

    assert_parses(".10"); // Leading decimal point
    assert_parses("10."); // Trailing decimal point

    assert_not_lexes("20.0n"); // BigInt literal with decimal point
    assert_not_lexes("10h"); // Number literal with an identifier after
    assert_not_lexes("012n"); // Implicit octal BigInt literal
    assert_not_lexes("10na"); // BigInt literal with a character after
    assert_not_lexes("0xGH"); // Hex literal with wrong characters
    
    assert_not_lexes("10_"); // Trailing underscore
    assert_not_lexes("10_.0"); // Underscore before decimal point
    assert_not_lexes("10._0"); // Underscore after decimal point
    assert_not_lexes("10_n"); // Underscore directly before n in BigInt literalW
    assert_not_lexes("0x.5"); // Decimal point after 0x
    
    assert_not_lexes("0x"); // Missing hex digits
    assert_not_lexes("0o"); // Missing octal digits
    assert_not_lexes("0b"); // Missing binary digits
    assert_not_lexes("0xn"); // Missing hex digits
    assert_not_lexes("0on"); // Missing octal digits
    assert_not_lexes("0bn"); // Missing binary digits
    assert_not_lexes("0x_10"); // Underscore before digits
    assert_not_lexes("0o_10"); // Underscore before digits
    assert_not_lexes("0b_10"); // Underscore before digits
    

    assert_lexes_only("0b0123"); // Binary literal with wrong characters - should be lexed as '0b01' '23'
    assert_lexes_only("0o123456789"); // Octal literal with wrong characters - should be lexed as '0o1234567' '89' 
    assert_lexes_only("10.10.10"); // Number with multiple decimal points - should be lexed as '10.10' '.10'
    assert_lexes_only("10..5"); // Number with consecutive decimal points - should be lexed as '10.' '.5'
}

/// Tests that string literals are parsed, and that invalid string literals are rejected
#[test]
fn test_string_literals() {
    assert_parses(r#" "Some string" "#);
    assert_parses(r#" "Some string with \b \f \n \r \t \v " "#);
    assert_parses(r#" "Single quotes in double quoted string: '' `` " "#);
    assert_parses(r#" 'Double quotes in single quoted string: "" `` ' "#);
    assert_parses(r#" `Double and single quotes in backtick string "" '' ` "#);
    assert_parses(r#" `Newlines in a backtick quoted string
    ` "#);
    assert_parses(r#" "Escaped double quotes: \" " "#);
    assert_parses(r#" 'Escaped single quotes: \' ' "#);
    assert_parses(r#" `Escaped backticks: \` ` "#);
    

    assert_not_lexes(r#" "Newline in double quoted string
    " "#);
    assert_not_lexes(r#" 'Newline in single quoted string
    ' "#);
    assert_not_lexes(r#" "Unclosed double quote string "#);
    assert_not_lexes(r#" 'Unclosed single quote string "#);
    assert_not_lexes(r#" `Unclosed backtick string "#);
}

/// Tests that 
#[test]
fn test_comments() {
    assert_parses(r#" // A line comment with no newline "#);
    assert_parses(r#" // A line comment with a trailing newline 
    "#);
    assert_parses(r#" // A line comment // This is still commented "#);
    assert_parses(r#" /* A block comment */ "#);
    assert_parses(r#" /* A block comment
    with a newline */ "#);

    assert_not_lexes(r#" /* A block comment with no end "#);

}