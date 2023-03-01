#![cfg(test)]

use crate::engine::{Gc, ProgramSource, Config};
use crate::lexer::Lexer;
use crate::{parser::Parser, Program};

mod array_literals;
mod value_literals;
mod object_literals;

/// Asserts that a given program lexes and parses
fn assert_parses(code: &str) {
    Program::from_console(code, Config::default()).expect("Program should have parsed");
}

/// Asserts that a given program does not lex
fn assert_not_lexes(code: &str) {
    let program = Gc::new(Program {
        source: ProgramSource::Console,
        program: code.chars().collect(),
        ast: None,
        config: Config::default(),
    });
    let lexer = Lexer::new();
    lexer
        .lex(&program)
        .expect_err("Program should not have lexed");
}

/// Asserts that a given program lexes but does not parse.
/// This is useful for constructs which can be tokenised but which are not semantically valid, e.g. 'id id'
fn assert_lexes_only(code: &str) {
    let program = Gc::new(Program {
        source: ProgramSource::Console,
        program: code.chars().collect(),
        ast: None,
        config: Config::default(),
    });
    let lexer = Lexer::new();
    let tokens = lexer.lex(&program).expect("Program should have lexed");
    Parser::parse(program, tokens).expect_err("Program should not have parsed");
}

/// Asserts that an empty program parses
#[test]
fn test_empty_program() {
    assert_parses("");
    assert_parses(" ");
    assert_parses(
        "
    ",
    );
}

#[test]
fn test_bracket_matching() {
    assert_parses("(a)");
    assert_parses("[]");
    assert_parses("{}");
    assert_parses("({})");
    assert_parses("([({})])");
    assert_parses("{[]}");
    assert_parses(
        "( [ ( [ ( [ ( [ 
            (( [[ (( [[ 
                ((( [[[ ((( [[[ 
                    (((( [[[[ ]]]] )))) 
                ]]] ))) ]]] )))
            ]] )) ]] ))
        ] ) ] ) ] ) ] )",
    );

    assert_lexes_only("({[{()}]})");
    assert_lexes_only("({[{()}]})");

    assert_not_lexes("(");
    assert_not_lexes("{");
    assert_not_lexes("[");
    assert_not_lexes(")");
    assert_not_lexes("}");
    assert_not_lexes("]");

    assert_not_lexes("())");
    assert_not_lexes("{})");
    assert_not_lexes("[])");
    assert_not_lexes("{{}");
    assert_not_lexes("[[]");
    assert_not_lexes("(()");

    assert_not_lexes("(]");
    assert_not_lexes("[}");
    assert_not_lexes("{)");

    assert_not_lexes("({)}");
    assert_not_lexes("[(])");

}
