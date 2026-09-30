mod address;
mod error;
mod fee;

use std::env;

use address::{known_program_name, Pubkey};
use error::ToolError;
use fee::{calculate_fee, lamports_to_sol};

// Переводит текст в целое число или возвращает ошибку
fn parse_u64(text: &str) -> Result<u64, ToolError> {
    text.parse::<u64>()
        .map_err(|_| ToolError::InvalidNumber(text.to_string()))
}

// Переводит текст в lamports или возвращает ошибку
fn to_lamports(text: &str) -> Result<f64, ToolError> {
    let sol = text
        .parse::<f64>()
        .map_err(|_| ToolError::InvalidNumber(text.to_string()))?;
    Ok(sol * 1_000_000_000.0)
}

// Выбирает команду и выполняет её
fn run(args: &[String]) -> Result<String, ToolError> {
    let command = args
        .get(1)
        .ok_or(ToolError::MissingArgument("команда".to_string()))?;

    match command.as_str() {
        "to-lamports" => {
            let text = args
                .get(2)
                .ok_or(ToolError::MissingArgument("число SOL".to_string()))?;
            let lamports = to_lamports(text)?;
            Ok(format!("{}", lamports))
        }
        "validate" => {
            let text = args
                .get(2)
                .ok_or(ToolError::MissingArgument("адрес".to_string()))?;
            let key = Pubkey::from_base58(text)?;
            let mut answer = format!("OK: корректный адрес Solana (32 байта)\n{}", key);
            if let Some(name) = known_program_name(text) {
                answer.push_str(&format!("\nИзвестная программа: {}", name));
            }
            Ok(answer)
        }
        "fee" => {
            let signatures = parse_u64(
                args.get(2)
                    .ok_or(ToolError::MissingArgument("число подписей".to_string()))?,
            )?;
            let cu_limit = parse_u64(
                args.get(3)
                    .ok_or(ToolError::MissingArgument("лимит CU".to_string()))?,
            )?;
            let price = parse_u64(
                args.get(4)
                    .ok_or(ToolError::MissingArgument("цена CU в micro-lamports".to_string()))?,
            )?;
            let fee = calculate_fee(signatures, cu_limit, price);
            Ok(format!(
                "Базовая комиссия: {} lamports\nПриоритетная комиссия: {} lamports\nИтого: {} lamports = {} SOL",
                fee.base,
                fee.priority,
                fee.total(),
                lamports_to_sol(fee.total())
            ))
        }
        other => Err(ToolError::UnknownCommand(other.to_string())),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(&args) {
        Ok(text) => println!("{}", text),
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            std::process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn week1_transfer_fee() {
        // Транзакция из недели 1: 562 CU, цена 133 333 334 micro-lamports
        let fee = calculate_fee(1, 562, 133_333_334);
        assert_eq!(fee.priority, 74_934);
        assert_eq!(fee.total(), 79_934);
    }

    #[test]
    fn whirlpool_swap_fee() {
        // Транзакция 3 из недели 2: сверху 5 lamports при лимите 43 560 CU
        let fee = calculate_fee(1, 43_560, 100);
        assert_eq!(fee.total(), 5_005);
    }

    #[test]
    fn lamports_to_sol_text() {
        assert_eq!(lamports_to_sol(79_934), "0.000079934");
    }
}