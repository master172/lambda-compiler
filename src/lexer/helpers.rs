pub fn is_valid_identifier_name(name: &str) -> bool {
    if name.is_empty() || name == "_" {
        return false;
    }
    let mut chars = name.chars();

    //check first character
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }

    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }

    true
}
