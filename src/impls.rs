//! implementation of String conversions

use super::*;

impl ConvertString for String {
    /// return true if the String is a reserved keyword
    fn is_keyword(&self) -> bool {
        KEYWORDS.contains(&self.as_ref())
    }

    /// transform String to PascalCase
    fn to_pascal_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        let mut last_uppercase = false;

        for c in self.chars() {
            if c.is_ascii_alphanumeric() {
                if capitalize_next || (c.is_uppercase() && !last_uppercase) {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c.to_ascii_lowercase());
                }
            } else {
                capitalize_next = true;
            }
            last_uppercase = c.is_uppercase();
        }

        result
    }

    /// transform String to snake_case
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        let mut last_uppercase = false;
        let mut last_underscore = false;

        for c in self.chars() {
            if c.is_uppercase() {
                if !result.is_empty() && !last_uppercase && !last_underscore {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());

                last_uppercase = true;
                last_underscore = false;
            } else if !c.is_ascii_alphanumeric() {
                if !last_underscore {
                    result.push('_');
                }
                last_uppercase = false;
                last_underscore = true;
            } else {
                result.push(c);
                last_uppercase = false;
                last_underscore = false;
            }
        }

        result
    }

    /// convert the String, if it is a reserved keyword. In that case add the prefix and an underscore
    fn to_valid_key(&self, prefix: &str) -> String {
        let mut new_name = self.replace(':', "_");

        new_name = new_name.to_snake_case();

        if new_name.is_keyword() {
            new_name = format!("{}_{}", prefix.to_string().to_snake_case(), &new_name);
        }

        new_name
    }

    /// given an XML like namespace string (ns:name), return the part after the colon
    fn remove_namespace(&self) -> String {
        if let Some(index) = self.find(':') {
            let extracted = &self[(index + 1)..];
            extracted.to_string()
        } else {
            self.clone()
        }
    }
}
