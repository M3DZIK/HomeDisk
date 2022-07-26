use std::io;

/// Custom functions implemented for Option<T>
pub trait OptionOkOrErr<T> {
    /// If the value is some return it, if not return Error
    fn ok_or_err(self, desc: &str) -> Result<T, io::Error>;
}

impl<T> OptionOkOrErr<T> for Option<T> {
    fn ok_or_err(self, desc: &str) -> Result<T, io::Error> {
        self.ok_or_else(|| io::Error::new(io::ErrorKind::Other, desc))
    }
}
