fn test_divisor(number: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None  // Return -1 if the divisor is zero, as division by zero is not allowed
    } else if number % divisor == 0 {
        Some(number / divisor)  // Return the result of number divided by divisor if it is a divisor
    } else {
        None  // Return -1 if the divisor is not a divisor of the number
    }
}

fn main() {
    let number = 10;
    let divisor = 5;

    let result = test_divisor(number, divisor);

    match result{
        Some(result) =>{
            println!("{} divided by {} equals {}", number, divisor, result);
        } ,
        None => {
            println!("{} is not divisible by {}", number, divisor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_zero() {
        assert_eq!(test_divisor(10, 0), None);  // Test division by zero
    }

    #[test]
    fn test_valid_division() {
        assert_eq!(test_divisor(10, 2), Some(5));   // Test valid division
    }

    #[test]
    fn test_invalid_division() {
        assert_eq!(test_divisor(10, 3), None);  // Test invalid divisor
    }

    #[test]
    fn test_negative_division() {
        assert_eq!(test_divisor(10, -2), Some(-5));  // Test valid division with a negative divisor
    }
}
