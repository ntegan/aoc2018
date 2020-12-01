
    use std::fmt;
    #[derive(Debug)]
    pub struct MyError;
    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "There is an error")
        }
    }
    impl std::error::Error for MyError {}
