use validator::Validate;

/// Custom validation utilities
pub fn validate_url(url: &str) -> Result<(), String> {
    url.parse::<reqwest::Url>()
        .map_err(|e| format!("Invalid URL: {}", e))
}

/// Validate that a string is not empty and within length bounds
pub fn validate_string_length(s: &str, min: usize, max: usize) -> Result<(), String> {
    let len = s.chars().count();
    
    if len < min {
        Err(format!("String must be at least {} characters", min))
    } else if len > max {
        Err(format!("String must be at most {} characters", max))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://localhost:3000").is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(validate_url("not-a-url").is_err());
        assert!(validate_url("").is_err());
    }

    #[test]
    fn test_validate_string_length() {
        assert!(validate_string_length("hello", 1, 10).is_ok());
        assert!(validate_string_length("a", 1, 10).is_ok());
        assert!(validate_string_length("hello world this is too long", 1, 10).is_err());
        assert!(validate_string_length("", 1, 10).is_err());
    }
}
