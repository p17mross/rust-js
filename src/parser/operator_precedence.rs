#[allow(dead_code)]
pub(crate) mod precedences {
    pub const GROUPING: usize = 18;

    pub const MEMBER_ACCESS: usize = 17;
    pub const OPTIONAL_CHAINING: usize = 17;
    pub const COMPUTED_MEMBER_ACCESS: usize = 17;
    pub const NEW_WITH_ARGUMENT_LIST: usize = 17;
    pub const FUNCTION_CALL: usize = 17;
    pub const ASSIGNMENT_TARGET: usize = 17;

    pub const NEW_WITHOUT_ARGUMENT_LIST: usize = 16;

    pub const POSTFIX: usize = 15;

    pub const UNARY_OPERATOR: usize = 14;
    pub const PREFIX: usize = 14;
    pub const DELETE: usize = 14;

    pub const EXPONENTIATION: usize = 13;

    pub const MULTIPLICATION: usize = 12;
    pub const DIVISION: usize = 12;
    pub const REMAINDER: usize = 12;

    pub const ADDITION: usize = 11;
    pub const SUBTRACTION: usize = 11;

    pub const SHIFT_LEFT: usize = 10;
    pub const SHIFT_RIGHT: usize = 10;
    pub const UNSIGNED_SHIFT_RIGHT: usize = 10;

    pub const LESS_THAN: usize = 9;
    pub const LESS_THAN_OR_EQUAL: usize = 9;
    pub const GREATER_THAN: usize = 9;
    pub const GREATER_THAN_OR_EQUAL: usize = 9;
    pub const IN: usize = 9;
    pub const INSTANCE_OF: usize = 9;

    pub const EQUALITY: usize = 8;
    pub const INEQUALITY: usize = 8;
    pub const STRICT_EQUALITY: usize = 8;
    pub const STRICT_INEQUALITY: usize = 8;

    pub const BITWISE_AND: usize = 7;

    pub const BITWISE_XOR: usize = 6;

    pub const BITWISE_OR: usize = 5;

    pub const LOGICAL_AND: usize = 4;

    pub const LOGICAL_OR: usize = 3;
    pub const NULLISH_COALESCING: usize = 3;

    pub const ASSIGNMENT: usize = 2;
    pub const TERNARY_OPERATOR: usize = 2;
    pub const ARROW_FUNTION: usize = 2;
    pub const YIELD: usize = 2;
    pub const SPREAD: usize = 2;

    pub const COMMA: usize = 1;

    pub const ANY_EXPRESSION: usize = 0;


}