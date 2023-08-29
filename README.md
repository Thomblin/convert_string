 # convert_string
 
 A trait for String-like types to check if a string is a reserved keyword,
 and convert it to a safe non-keyword if so. Offers some type conversions as well
 
 Only strict and reserved keywords are checked against; weak keywords are not include
 
 You can add this dependency with:
 
 ```toml
 [dependencies]
 convert_string = "0.1"
 ```
 
 ## Example
 
 ```rust
 use convert_string::ConvertString;;
 
 assert_eq!("r_type", String::from("type").to_valid_key(&String::from("r")));
 assert_eq!("foo", String::from("ns:foo").remove_namespace());
 assert_eq!("YdTax", String::from("yd_tax").to_pascal_case());
 assert_eq!("yd_tax", String::from("YdTax").to_snake_case());
 
 ```
 
 ## Rust Editions
 
 By default, the keywords added in Rust Edition 2018 are included in the list of chec
 This can be disabled with `default-features = false` in your Cargo.toml.
 
 ```toml
 [dependencies]
 convert_string = { version = "0.1", default-features = false }
 ```
 
 Future Rust editions may add new keywords, and this crate will be updated to reflect
 (Or you can create an issue on github if I don't.)
 
 # Credit
 
 Huge parts of this library are taken from <https://github.com/JoelCourtney/check_key
 
 License: MIT OR Apache-2.0
 
 # Source
 
 <https://github.com/Thomblin/convert_string>