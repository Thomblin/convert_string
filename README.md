 # convert_string
 
[<img alt="github" src="https://img.shields.io/badge/github-thomblin/convert_string-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/thomblin/convert_string)
[<img alt="crates.io" src="https://img.shields.io/crates/v/convert_string?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/convert_string)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/convert_string?logo=docs.rs&labelColor=555555" height="20">](https://docs.rs/convert_string)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Thomblin/convert_string/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/thomblin/convert_string/actions?query=branch%3Amain)

 A trait for String-like types to check if a string is a reserved keyword,
 and convert it to a safe non-keyword if so. Offers some type conversions as well
 
 Only strict and reserved keywords are checked against; weak keywords are not include
 
 You can add this dependency with:
 
 ```toml
 [dependencies]
 convert_string = "0.1.3"
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
 convert_string = { version = "0.1.3", default-features = false }
 ```
 
 Future Rust editions may add new keywords, and this crate will be updated to reflect
 (Or you can create an issue on github if I don't.)
 
 # Credit
 
 Huge parts of this library are taken from <https://github.com/JoelCourtney/check_keyword>
  
 # Source
 
 License: Apache-2.0

 <https://github.com/Thomblin/convert_string>