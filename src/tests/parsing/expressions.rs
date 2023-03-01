//! Tests relating to expressions

use super::*;

/// Tests that property lookups are parsed
#[test]
fn test_property_lookups() {
    assert_parses("a.b"); // Property lookups
    assert_parses("a['string']");
    assert_parses("a?.b");

    assert_parses("a.b.c"); // Property lookups can be chained
    assert_parses("a.b[10]");
    assert_parses("a.b['string']");
    assert_parses("a.b?.c");
    assert_parses("a?.b.c");

    assert_parses("(a + b).c"); // The lhs can be any expression
    assert_parses("(a + b)[10]");
    assert_parses("(a + b)?.c");
    assert_parses("'string'.a");

    assert_lexes_only("a.10"); // Property lookup rhs must be an identifier
    assert_lexes_only("a.'property'");
    assert_lexes_only("a.(b)");
    assert_lexes_only("a.");

    assert_not_lexes("10.a"); // Number literals can't be the lhs of a property lookup...
    assert_parses("0x10.a"); // Unless they're hex, octal, or binary literals
    assert_parses("0o10.a");
    assert_parses("010.a");
    assert_parses("0b10.a");
}

/// Tests that function calls are parsed
#[test]
fn test_function_calls() {
    assert_parses("f()"); // Function call
    assert_parses("f(10)");
    assert_parses("f(10, 20, 30)");
    
    assert_parses("a.f(10)"); // The lhs can be any expression
    assert_parses("a(10)(10)"); // Chained function calls

    assert_parses("(a + b)(10)"); // These would fail at runtime but it should still be parsed
    assert_parses("[a, b](10)");
    assert_parses("'string'(10)");

    assert_lexes_only("()()"); // Empty expression
    assert_lexes_only("f(())"); // Empty expression
}