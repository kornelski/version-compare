//! Version part module.
//!
//! A module that provides the `VersionPart` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

/// Enum of version string parts.
///
/// Each version string is broken down into these version parts when being parsed to a `Version`.
#[derive(Debug, PartialEq)]
pub enum VersionPart<'a> {

    /// Numeric part, most common in version strings.
    /// Holds the numerical value.
    Number(i32),

    /// A text part.
    /// These parts usually hold text with an yet unknown definition.
    /// Holds the string slice.
    Text(&'a str)
}
