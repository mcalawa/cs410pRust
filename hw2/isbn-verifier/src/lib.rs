/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn10 = &isbn.trim().replace("-", "");
    if isbn10.len() != 10 {
        return false
    }
    else {
        let mut digits = isbn10.chars();
        let mut total = 0;
        for index in 0..9 {
            let current = digits.next().unwrap_or_else(|| 'A');
            if current.is_numeric() {
                total += current.to_digit(10).unwrap() * (10 - index);
            }
            else {
                return false
            }
        }

        if isbn10.ends_with("X") {
            total += 10;
        }
        else {
            let last = digits.next().unwrap_or_else(|| 'A');
            if last.is_numeric() {
                total += last.to_digit(10).unwrap();
            }
            else {
                return false
            }
        }

        if total % 11 == 0 {
            return true
        }
        else {
            return false
        }
    }
}
