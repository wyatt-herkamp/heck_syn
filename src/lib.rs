mod rename_to_lit_str;
#[doc(inline)]
pub use rename_to_lit_str::RenameToLitStr;
use std::str::FromStr;
use strum::{EnumString, EnumVariantNames, VariantNames};
use syn::parse::Parse;

/// A syn parseable enum that represents the case to convert to
///
/// This enum is used to determine the case to convert to
///
/// Parses from Either an Ident or a LitStr

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, EnumVariantNames)]
pub enum RenameWith {
    /// 'camelCase' is primary name
    ///
    /// Other accepted names are 'lowerCamelCase'
    #[strum(serialize = "camelCase", to_string = "lowerCamelCase")]
    LowerCamelCase,
    /// 'UpperCamelCase' is primary name
    UpperCamelCase,
    /// 'PascalCase' is primary name
    #[strum(to_string = "PascalCase")]
    Pascal,
    /// `snake_case` is primary name
    ///
    /// Other accepted names are 'lower_snake_case'
    #[strum(serialize = "lower_snake_case", to_string = "snake_case")]
    Snake,
    /// `UPPER_SNAKE_CASE` is primary name
    ///
    /// Other accepted names are 'SCREAMING_SNAKE_CASE'
    #[strum(serialize = "UPPER_SNAKE_CASE", to_string = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,
    /// `kebab-case` is primary name
    ///
    /// Other accepted names are 'lower-kebab-case'
    #[strum(serialize = "lower-kebab-case", to_string = "kebab-case")]
    Kebab,
    /// `SCREAMING-KEBAB-CASE` is primary name
    ///
    /// Other accepted names are 'UPPER-KEBAB-CASE'
    #[strum(serialize = "UPPER-KEBAB-CASE", to_string = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
    /// UPPERCASE
    ///
    /// Converts the case by converting to camelCase then calling to_uppercase
    #[strum(serialize = "UPPERCASE")]
    UpperCase,
    /// lowercase
    ///
    /// Converts the case by converting to camelCase then calling to_lowercase
    #[strum(serialize = "lowercase")]
    LowerCase,
}

impl Parse for RenameWith {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Accept Either an Ident or a LitStr
        let name = if input.peek(syn::Ident) {
            input.parse::<syn::Ident>()?.to_string()
        } else if input.peek(syn::LitStr) {
            input.parse::<syn::LitStr>()?.value()
        } else {
            return Err(input.error("Expected either an Ident or a LitStr"));
        };

        // Convert to RenameWith
        RenameWith::from_str(&name).map_err(|_| {
            input.error(format!(
                "Expected one of the following: {}",
                RenameWith::VARIANTS.join(", ")
            ))
        })
    }
}
