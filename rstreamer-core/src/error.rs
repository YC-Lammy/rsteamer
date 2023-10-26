

pub enum Error{
    DuplicatedLink,
    DuplicatedSource,
    UnsupportedFormat
}

impl Error{
    pub fn error<E: std::error::Error>(e: E) -> Self{
        todo!()
    }
}