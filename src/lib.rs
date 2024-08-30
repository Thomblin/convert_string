//! # convert_string
//!
//! A trait for String-like types to check if a string is a reserved keyword,
//! and convert it to a safe non-keyword if so. Offers some type conversions as well
//!
//! Only strict and reserved keywords are checked against; weak keywords are not included.
//!
//! You can add this dependency with:
//!
//! ```toml
//! [dependencies]
//! convert_string = "0.1.4"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use convert_string::ConvertString;;
//!
//! assert_eq!("r_type", String::from("type").to_valid_key(&String::from("r")));
//! assert_eq!("foo", String::from("ns:foo").remove_namespace());
//! assert_eq!("YdTax", String::from("yd_tax").to_pascal_case());
//! assert_eq!("yd_tax", String::from("YdTax").to_snake_case());
//!
//! ```
//!
//! ## Rust Editions
//!
//! By default, the keywords added in Rust Edition 2018 are included in the list of checked keywords.
//! This can be disabled with `default-features = false` in your Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! convert_string = { version = "0.1", default-features = false }
//! ```
//!
//! Future Rust editions may add new keywords, and this crate will be updated to reflect that.
//! (Or you can create an issue on github if I don't.)
//!
//! # Credit
//!
//! Huge parts of this library are taken from <https://github.com/JoelCourtney/check_keyword>
//!
//! License: MIT OR Apache-2.0
//!
//! # Source
//!
//! <https://github.com/Thomblin/convert_string>
mod impls;

/// definition of all available conversion functions
#[macro_use]
mod arr_macro;
pub trait ConvertString {
    /// return true if the String is a reserved keyword
    ///
    /// Example
    /// ```   
    /// use convert_string:: ConvertString;
    ///      
    /// assert_eq!(false, String::from("name").is_keyword());
    /// assert_eq!(true, String::from("type").is_keyword());
    /// ```   
    fn is_keyword(&self) -> bool;

    /// transform String to PascalCase  
    ///
    /// Example
    /// ```   
    /// use convert_string:: ConvertString;
    ///      
    /// assert_eq!("Name", String::from("name").to_pascal_case());
    /// assert_eq!("WithSpaces", String::from("with spaces").to_pascal_case());
    /// assert_eq!("YdTax", String::from("yd_tax").to_pascal_case());
    /// assert_eq!("EdgeCase", String::from("edge-case").to_pascal_case());
    /// assert_eq!("YdTax", String::from("YdTax").to_pascal_case());
    /// assert_eq!("VendorRateId", String::from("VendorRateID").to_pascal_case());
    /// assert_eq!("MvciModuleDescription", String::from("MVCI_MODULE_DESCRIPTION").to_pascal_case());
    /// assert_eq!("HttpresponseCode", String::from("HTTPResponseCode").to_pascal_case());
    /// assert_eq!("Pintype", String::from("PINTYPE").to_pascal_case());
    /// assert_eq!("КоммерческаяИнформация", String::from("КоммерческаяИнформация").to_pascal_case());
    /// assert_eq!("Коммерческаяинформация", String::from("КОММЕРЧЕСКАЯИНФОРМАЦИЯ").to_pascal_case());
    /// assert_eq!("Коммерческаяинформация", String::from("коммерческаяинформация").to_pascal_case());
    /// ```   
    fn to_pascal_case(&self) -> String;

    /// transform String to snake_case
    ///
    /// Example
    /// ```
    /// use convert_string:: ConvertString;
    ///      
    /// assert_eq!("name", String::from("name").to_snake_case());
    /// assert_eq!("with_spaces", String::from("with spaces").to_snake_case());
    /// assert_eq!("yd_tax", String::from("yd_tax").to_snake_case());
    /// assert_eq!("edge_case", String::from("edge-case").to_snake_case());
    /// assert_eq!("yd_tax", String::from("YdTax").to_snake_case());
    /// assert_eq!("vendor_rate_id", String::from("VendorRateID").to_snake_case());
    /// assert_eq!("_id_context", String::from("_ID_Context").to_snake_case());
    /// assert_eq!("mvci_module_description", String::from("MVCI_MODULE_DESCRIPTION").to_snake_case());
    /// assert_eq!("pintype", String::from("PINTYPE").to_snake_case());
    /// assert_eq!("коммерческая_информация", String::from("КоммерческаяИнформация").to_snake_case());
    /// ```
    fn to_snake_case(&self) -> String;

    /// convert the String, if it is a reserved keyword. In that case add the prefix and an underscore
    ///
    /// Example
    /// ```
    /// use convert_string:: ConvertString;
    ///
    /// assert_eq!("not_a_keyword", String::from("not_a_keyword").to_valid_key(&String::from("r")));  
    /// assert_eq!("no_keyword", String::from("NoKeyword").to_valid_key(&String::from("r")));      
    /// assert_eq!("r_type", String::from("type").to_valid_key(&String::from("r")));    
    /// assert_eq!("r_type", String::from("Type").to_valid_key(&String::from("r")));
    /// assert_eq!("r_abstract", String::from("abstract").to_valid_key(&String::from("r")));
    /// ```
    fn to_valid_key(&self, prefix: &str) -> String;

    /// given an XML like namespace string (ns:name), return the part after the colon
    ///
    /// Example
    /// ```
    /// use convert_string:: ConvertString;
    ///
    /// assert_eq!("name", String::from("name").remove_namespace());
    /// assert_eq!("name", String::from("ns:name").remove_namespace());
    /// ```
    fn remove_namespace(&self) -> String;
}

// defined keywords, taken from <https://github.com/JoelCourtney/check_keyword>
arr!(static KEYWORDS: [&'static str; _] = [

    // STRICT, 2015

    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",

    // STRICT, 2018

    #[cfg(feature = "2018")] "async",
    #[cfg(feature = "2018")] "await",
    #[cfg(feature = "2018")] "dyn",

    // RESERVED, 2015

    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",

    // RESERVED, 2018

    #[cfg(feature = "2018")] "try"

]);
