use std::fmt;

// Список всех ошибок нашей программы
#[derive(Debug)]
pub enum ToolError {
    InvalidNumber(String),
    MissingArgument(String),
    UnknownCommand(String),
    InvalidBase58(String),
    InvalidLength(usize),
}

// Как каждую ошибку показывать человеку
impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToolError::InvalidNumber(s) => write!(f, "это не число: {}", s),
            ToolError::MissingArgument(s) => write!(f, "не хватает аргумента: {}", s),
            ToolError::UnknownCommand(s) => write!(f, "неизвестная команда: {}", s),
            ToolError::InvalidBase58(s) => write!(f, "это не base58: {}", s),
            ToolError::InvalidLength(n) => write!(f, "в адресе {} байт, а нужно 32", n),
        }
    }
}