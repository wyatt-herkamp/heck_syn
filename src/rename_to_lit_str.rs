use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToUpperCamelCase,
};
use syn::{Ident, LitStr};

use crate::RenameWith;

/// Can be renamed and converted to a LitStr
pub trait RenameToLitStr {
    fn into_lit_str(&self) -> LitStr;
    /// Convert to target case
    fn to_upper_camel_case(&self) -> LitStr;
    fn to_lower_camel_case(&self) -> LitStr;
    /// Convert to pascal case
    fn to_pascal_case(&self) -> LitStr;
    /// Convert to snake case
    fn to_snake_case(&self) -> LitStr;
    /// Convert to screaming snake case
    fn to_screaming_snake_case(&self) -> LitStr;
    /// Convert to kebab case
    fn to_kebab_case(&self) -> LitStr;
    /// Convert to screaming kebab case
    fn to_screaming_kebab_case(&self) -> LitStr;
    /// Convert to target case
    fn to_uppercase(&self) -> LitStr;
    fn to_lowercase(&self) -> LitStr;

    #[inline]
    fn to_case(&self, case: RenameWith) -> LitStr {
        match case {
            RenameWith::Pascal => self.to_pascal_case(),
            RenameWith::Snake => self.to_snake_case(),
            RenameWith::ScreamingSnake => self.to_screaming_snake_case(),
            RenameWith::Kebab => self.to_kebab_case(),
            RenameWith::ScreamingKebab => self.to_screaming_kebab_case(),
            RenameWith::UpperCase => self.to_uppercase(),
            RenameWith::LowerCase => self.to_lowercase(),
            RenameWith::LowerCamelCase => self.to_lower_camel_case(),
            RenameWith::UpperCamelCase => self.to_upper_camel_case(),
        }
    }
    /// Convert to target case or return original string
    #[inline]
    fn to_case_option(&self, case: Option<RenameWith>) -> LitStr {
        match case {
            None => self.into_lit_str(),
            Some(case) => self.to_case(case),
        }
    }
}

impl RenameToLitStr for Ident {
    fn to_pascal_case(&self) -> LitStr {
        let value = self.to_string().to_pascal_case();
        LitStr::new(&value, self.span())
    }

    fn to_snake_case(&self) -> LitStr {
        let value = self.to_string().to_snake_case();
        LitStr::new(&value, self.span())
    }

    fn to_screaming_snake_case(&self) -> LitStr {
        let value = self.to_string().to_shouty_snake_case();
        LitStr::new(&value, self.span())
    }

    fn to_kebab_case(&self) -> LitStr {
        let value = self.to_string().to_kebab_case();
        LitStr::new(&value, self.span())
    }

    fn to_screaming_kebab_case(&self) -> LitStr {
        let value = self.to_string().to_shouty_kebab_case();
        LitStr::new(&value, self.span())
    }

    fn to_uppercase(&self) -> LitStr {
        let value = self.to_string().to_uppercase();
        LitStr::new(&value, self.span())
    }

    fn to_lowercase(&self) -> LitStr {
        let value = self.to_string().to_lowercase();
        LitStr::new(&value, self.span())
    }

    fn into_lit_str(&self) -> LitStr {
        LitStr::new(&self.to_string(), self.span())
    }

    fn to_upper_camel_case(&self) -> LitStr {
        let value = self.to_string().to_upper_camel_case();
        LitStr::new(&value, self.span())
    }

    fn to_lower_camel_case(&self) -> LitStr {
        let value = self.to_string().to_lower_camel_case();
        LitStr::new(&value, self.span())
    }
}

impl RenameToLitStr for LitStr {
    fn to_pascal_case(&self) -> LitStr {
        let value = self.value().to_pascal_case();
        LitStr::new(&value, self.span())
    }

    fn to_snake_case(&self) -> LitStr {
        let value = self.value().to_snake_case();
        LitStr::new(&value, self.span())
    }

    fn to_screaming_snake_case(&self) -> LitStr {
        let value = self.value().to_shouty_snake_case();
        LitStr::new(&value, self.span())
    }

    fn to_kebab_case(&self) -> LitStr {
        let value = self.value().to_kebab_case();
        LitStr::new(&value, self.span())
    }

    fn to_screaming_kebab_case(&self) -> LitStr {
        let value = self.value().to_shouty_kebab_case();
        LitStr::new(&value, self.span())
    }

    fn to_uppercase(&self) -> LitStr {
        let value = self.value().to_uppercase();
        LitStr::new(&value, self.span())
    }

    fn to_lowercase(&self) -> LitStr {
        let value = self.value().to_lowercase();
        LitStr::new(&value, self.span())
    }

    fn into_lit_str(&self) -> LitStr {
        self.clone()
    }

    fn to_upper_camel_case(&self) -> LitStr {
        let value = self.value().to_upper_camel_case();
        LitStr::new(&value, self.span())
    }

    fn to_lower_camel_case(&self) -> LitStr {
        let value = self.value().to_lower_camel_case();
        LitStr::new(&value, self.span())
    }
}
