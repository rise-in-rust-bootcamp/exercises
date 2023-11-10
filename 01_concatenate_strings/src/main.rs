fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rise In Rust Bootcamp!");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(first: &str, second: &str) -> String {
    let mut result: String = String::new();
    result.push_str(first);
    result.push_str(second);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_strings() {
        let string1 = String::from("Hello, ");
        let string2 = String::from("Rise In Rust Bootcamp!");

        let result = concatenate_strings(&string1, &string2);

        assert_eq!(result, "Hello, Rise In Rust Bootcamp!");
    }
}
