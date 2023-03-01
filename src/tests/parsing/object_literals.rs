//! Tests for object literals

use super::*;

#[test]
fn test_object_literals() {
    // Brackets are needed in these tests to prevent the expressions from being parsed as blocks

    assert_parses("({})"); // Empty object
    assert_parses("({a: 10})"); // Simple properties
    assert_parses("({a: 10, b: 20})");
    assert_parses(r#" ({a: "A string"}) "#);
    assert_parses("({a: {another_object: 10}})");

    assert_parses("({a})"); // Shorthand properties
    assert_parses("({a, b, c: 20, d})");
    assert_parses("({10: 10})"); // Numbers as keys
    assert_parses(r#" ({["a"]: 10}) "#); // Computed properties
    assert_parses("({...{a: 10}})"); // Spread

    assert_parses("({a: 10,})"); // Trailing commas are allowed
    assert_parses("({a,})");
    assert_parses(r#" ({a, ...{b: 20, c: 30}, ...{["d"]: 40}}) "#);

    assert_lexes_only("({10})"); // Shorthand properties can't be numbers
    assert_lexes_only(r#" ({"string"}) "#); // Shorthand properties can't be strings
    assert_lexes_only("({,a: 10})"); // Leading commas are not allowed
    assert_lexes_only("({,})"); // Only comma
    assert_lexes_only("({a,,b})"); // Multiple commas
}
