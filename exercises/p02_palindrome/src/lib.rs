pub fn is_palindrome(input: &str) -> bool {
    let mut chars = input.chars();

    loop {
        let front = chars.next();
        let back = chars.next_back();

        if front.is_none() || back.is_none() {
            return true;
        }

        if front != back {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("abba"));
        assert!(is_palindrome("a😊a"));
        assert!(is_palindrome("öö"));
        assert!(is_palindrome("été"));
        assert!(is_palindrome("abcba"));
        assert!(!is_palindrome("abcd"));
        assert!(!is_palindrome("abc😊d😊cb"));
    }
}
