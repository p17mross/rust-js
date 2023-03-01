//! Tests for array literals

use super::*;

/// Tests for parsing of array literals
#[test]
fn test_array_literals() {
    assert_parses("[]");
    assert_parses("[10]");
    assert_parses("[10 + 20]");
    assert_parses("[0, 1, 2, 3]");
    assert_parses("[0, (1 + 2), 3, 4]");
    assert_parses("[test, (a + b), test2]");
    assert_parses("[[[[[[[[[[[[[[]]]]]]]]]]]]]]");

    assert_parses("[10,]"); // Trailing comma
    assert_parses("[10 ,,, 20]"); // Multiple commas
    assert_parses("[10,,,,,]"); // Multiple trailing commas
    assert_parses("[,10]"); // Leading comma
    assert_parses("[,10]"); // Multiple leading commas
    assert_parses("[,,]"); // Only commas

    assert_parses("[...[]]"); // Spread
    assert_parses("[...a]");
    assert_parses("[...a, b, c, ...d]");
    assert_parses("[...[...[...a], ...[b]], ...[c]]");

    assert_lexes_only("[a b]");
    assert_lexes_only("[()]");
    assert_lexes_only("[(a + b) 10]");
}
