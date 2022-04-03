extern crate proc_macro;

mod pattern;

use pattern::Pattern;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(Bitrange, attributes(BitrangeMask, BitrangeSize))]
pub fn bitrange(input: TokenStream) -> TokenStream {
    let pattern = Pattern::from_stream(input).expect("Could not parse mask");

    let str = format!(
        r#"
impl {struct_name} {{
    {get_mask}
    {get_offset}
    {get_default_mask}
    {get_default_value}
}}
"#,
        struct_name = pattern.struct_name,
        get_mask = generate_mask(&pattern),
        get_offset = generate_offset(&pattern),
        get_default_mask = generate_default_mask(&pattern),
        get_default_value = generate_default_value(&pattern),
    );

    // println!("{}", str);
    TokenStream::from_str(&str).unwrap()
}

fn generate_mask(pattern: &Pattern) -> String {
    let mut case_statements = String::new();
    let mut examples = String::new();
    for token in &pattern.tokens {
        let mask = pattern.get_token_mask(*token);
        case_statements += &format!("            \"{}\" => {},\n", token, mask);
        examples += &format!(
            "    /// assert_eq!({}, {}::__bitrange_get_mask(\"{}\"));\n",
            mask, pattern.struct_name, token
        );
    }

    format!(
        r#"
    /// Create a mask based on a given format and a character
    /// This will map all the bits that match the given character, to 1
    /// All other bits will be set to 0
    /// 
    /// ```
{examples}
    /// ```
    pub fn __bitrange_get_mask(c: &str) -> {size} {{
        match c {{
{case_statements}
            _ => panic!("Invalid mask character (__bitrange_get_mask): {{:?}}", c),
        }}
    }}
"#,
        case_statements = case_statements,
        examples = examples,
        size = pattern.size,
    )
}

fn generate_offset(pattern: &Pattern) -> String {
    let mut examples = String::new();
    let mut case_statements = String::new();
    for token in &pattern.tokens {
        let offset = pattern.get_token_offset(*token);
        let mask = pattern.get_token_mask(*token);
        case_statements += &format!("            \"{}\" => {}, // {}\n", token, offset, mask);
        examples += &format!(
            "    /// assert_eq!({}, {}::__bitrange_get_offset(\"{}\")); // {}\n",
            offset, pattern.struct_name, token, mask
        );
    }

    format!(
        r#"
    /// Return the offset of a given character in a format.
    /// This is the amount of least-significant bits in the proc_mask that are 0.
    /// 
    /// e.g. the offset for 0b0000_1100 would be 2
    /// 
    /// ```
{examples}
    /// ```
    pub fn __bitrange_get_offset(c: &str) -> usize {{
        match c {{
{case_statements}
            _ => panic!("Invalid mask character (__bitrange_get_offset): {{:?}})", c),
        }}
    }}
    "#,
        case_statements = case_statements,
        examples = examples
    )
}

fn generate_default_mask(pattern: &Pattern) -> String {
    format!(
        r#"
    /// Return the default mask of a format.
    /// This is a mask with all the fields set that are either `0` or `1`
    /// 
    /// ```
    /// assert_eq!({result}, {struct_name}::__bitrange_get_default_mask());
    /// ```
    pub fn __bitrange_get_default_mask() -> {size} {{
        {result}
    }}
"#,
        struct_name = pattern.struct_name,
        result = pattern.get_default_mask(),
        size = pattern.size,
    )
}

fn generate_default_value(pattern: &Pattern) -> String {
    format!(
        r#"
    /// Returns the default value of a format
    /// This is a value with 1 for every `1` in the format
    /// 
    /// ```
    /// assert_eq!({result}, {struct_name}::__bitrange_get_default_value());
    /// ```
    pub fn __bitrange_get_default_value() -> {size} {{
        {result}
    }}
"#,
        result = pattern.get_default_value(),
        struct_name = pattern.struct_name,
        size = pattern.size,
    )
}
