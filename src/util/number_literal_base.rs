#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A type representing a base for a numerical literal.
/// Includes binary, octal, decimal, and hexadecimal.
pub enum NumberLiteralBase {
    /// Binary, using a `0b` prefix
    Binary,
    /// Octal, using a `0o` prefix
    Octal,
    /// Octal, using only a `0` prefix.
    /// This is different from [`Octal`][NumberLiteralBase::Octal] as it is not valid in strict mode and can't be used in BigInt literals.
    OctalImplicit,
    /// Decimal, using no prefix
    Decimal,
    /// Hexadecimal, using a `0x` prefix
    Hex,
}

impl NumberLiteralBase {
    /// Gets the valid chars for a given base
    pub const fn get_chars(self) -> &'static str {
        match self {
            Self::Binary => "01",
            Self::Octal | Self::OctalImplicit => "01234567",
            Self::Decimal => "0123456879",
            Self::Hex => "0123456789ABCDEFabcdef",
        }
    }

    /// Gets the base as a u32
    pub const fn get_radix(self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Octal | Self::OctalImplicit => 8,
            Self::Decimal => 10,
            Self::Hex => 16,
        }
    }

    /// Gets the name of the base
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Octal | Self::OctalImplicit => "octal",
            Self::Decimal => "decimal",
            Self::Hex => "hexadecimal",
        }
    }

    /// Gets what a literal must start with to have this base
    pub const fn get_start(self) -> &'static str {
        match self {
            Self::Binary => "0b",
            Self::Octal | Self::OctalImplicit => "0o",
            Self::Decimal => "",
            Self::Hex => "0x",
        }
    }
}
