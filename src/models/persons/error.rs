pub const ERR_PERSON_NOT_FOUND: u8 = 1;

#[derive(Debug)]
pub struct PersonError {
    pub code: u8,
    pub message: String,
}

impl std::fmt::Display for PersonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Code: {}, Message: {}", self.code, self.message)
    }
}

impl std::error::Error for PersonError {}