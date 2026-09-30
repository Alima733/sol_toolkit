use std::fmt;

use crate::error::ToolError;

// Публичный ключ Solana: ровно 32 байта
pub struct Pubkey([u8; 32]);

impl Pubkey {
    // Превращает текст в Pubkey или возвращает ошибку
    pub fn from_base58(text: &str) -> Result<Pubkey, ToolError> {
        let bytes = bs58::decode(text)
            .into_vec()
            .map_err(|e| ToolError::InvalidBase58(e.to_string()))?;
        let array: [u8; 32] = bytes
            .try_into()
            .map_err(|v: Vec<u8>| ToolError::InvalidLength(v.len()))?;
        Ok(Pubkey(array))
    }
}

// Как печатать Pubkey (обратно в base58)
impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

// Option: имя программы есть (Some) или нет (None)
pub fn known_program_name(address: &str) -> Option<&'static str> {
    match address {
        "11111111111111111111111111111111" => Some("System Program"),
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => Some("Token Program"),
        "ComputeBudget111111111111111111111111111111" => Some("Compute Budget Program"),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_known_program() {
        assert_eq!(
            known_program_name("11111111111111111111111111111111"),
            Some("System Program")
        );
        assert_eq!(known_program_name("abc"), None);
    }

    #[test]
    fn accepts_valid_address() {
        let key = Pubkey::from_base58("11111111111111111111111111111111").unwrap();
        assert_eq!(key.to_string(), "11111111111111111111111111111111");
    }

    #[test]
    fn rejects_short_address() {
        let result = Pubkey::from_base58("abc");
        assert!(matches!(result, Err(ToolError::InvalidLength(3))));
    }
}