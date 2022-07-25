use homedisk_types::errors::{FsError, ServerError};

/// Validate path param provided in the request
pub fn validate_path(path: &str) -> Result<(), ServerError> {
    // `path` can't contain `..`
    // to prevent attack attempts because by using a `..` you can access the previous folder
    if path.contains("..") {
        return Err(ServerError::FsError(FsError::ReadDirectory(
            "the `path` can't contain `..`".to_string(),
        )));
    }

    // `path` can't contain `~`
    // to prevent attack attempts because `~` can get up a directory on `$HOME`
    if path.contains('~') {
        return Err(ServerError::FsError(FsError::ReadDirectory(
            "the `path` can't not contain `~`".to_string(),
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path() {
        // Successfully
        assert!(validate_path("Directory/path/to/test.png").is_ok());
        assert!(validate_path("/test.png").is_ok()); // `/` doesn't point to the system root
        assert!(validate_path("./test.png").is_ok());

        // Errors
        assert!(validate_path("../../test.png").is_err());
        assert!(validate_path("../test.png").is_err());
        assert!(validate_path("~/test.png").is_err());
    }
}
