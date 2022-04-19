#[macro_export]
macro_rules! option_return {
    ($a:expr,$b:expr) => {
        match $a {
            Some(x) => Ok(x),
            None => {
                let err = std::io::Error::new(std::io::ErrorKind::Other, $b);
                Err(anyhow::Error::from(err))
            }
        }
    };
}
