/// The Operation enum represents different mathematical operations.
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    let first_number: f64 = read_number_input("Enter the first number:");
    let operator = read_operation_input("Enter the operation (+ -, *, /):");
    let second_number: f64 = read_number_input("Enter the second number:");

    let operation = match operator {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operator"),
    };

    println!("Result: {}", calculate(operation));
}

/// Calculates the result of the given operation.
///
/// # Arguments
///
/// * `operation` - The operation to be calculated.
///
/// # Returns
///
/// The result of the calculation.
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn read_number_input(prompt: &str) -> f64 {
    let input = read_user_input(prompt);
    input.trim().parse().expect("Invalid number")
}

fn read_operation_input(prompt: &str) -> char {
    let input = read_user_input(prompt);
    input.trim().chars().next().expect("Invalid operation")
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_with_addition() {
        let operation = Operation::Add(2.0, 3.0);
        let result = calculate(operation);

        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_calculate_with_subtraction() {
        let operation = Operation::Subtract(5.0, 3.0);
        let result = calculate(operation);

        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_calculate_with_multiplication() {
        let operation = Operation::Multiply(2.0, 3.0);
        let result = calculate(operation);

        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_calculate_with_division() {
        let operation = Operation::Divide(6.0, 3.0);
        let result = calculate(operation);

        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_calculate_with_division_by_zero() {
        let operation = Operation::Divide(6.0, 0.0);
        let result = calculate(operation);

        assert_eq!(result, std::f64::INFINITY);
    }
}
