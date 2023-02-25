#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A type representing a base for a numerical literal.
/// Includes binary, octal, decimal, and hexadecimal.
pub enum NumberLiteralBase {
    Binary,
    Octal,
    Decimal,
    Hex,
}

impl NumberLiteralBase {
    /// Gets the valid chars for a given base
    pub const fn get_chars(self) -> &'static str {
        match self {
            Self::Binary => "01",
            Self::Octal => "01234567",
            Self::Decimal => "0123456879",
            Self::Hex => "0123456789ABCDEFabcdef",
        }
    }

    /// Gets the base as a u32
    pub const fn get_radix(self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Decimal => 10,
            Self::Hex => 16,
        }
    }

    /// Gets the name of the base
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Octal => "octal",
            Self::Decimal => "decimal",
            Self::Hex => "hexadecimal",
        }
    }

    /// Gets what a literal must start with to have this base
    pub const fn get_start(self) -> &'static str {
        match self {
            Self::Binary => "0b",
            Self::Octal => "0o",
            Self::Decimal => "",
            Self::Hex => "0x",
        }
    }
}
