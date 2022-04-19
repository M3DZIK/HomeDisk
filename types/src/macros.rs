#[macro_export]
macro_rules! option_return {
    ($variable:expr,$err_desc:expr) => {
        $variable.ok_or(std::io::Error::new(std::io::ErrorKind::Other, $err_desc))
    };
}
