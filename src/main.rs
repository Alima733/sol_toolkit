use std::env;
use std::fmt;

// Список всех ошибок нашей программы
enum ToolError {
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

// Публичный ключ Solana: ровно 32 байта
struct Pubkey([u8; 32]);

impl Pubkey {
    // Превращает текст в Pubkey или возвращает ошибку
    fn from_base58(text: &str) -> Result<Pubkey, ToolError> {
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
fn known_program_name(address: &str) -> Option<&'static str> {
    match address {
        "11111111111111111111111111111111" => Some("System Program"),
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => Some("Token Program"),
        "ComputeBudget111111111111111111111111111111" => Some("Compute Budget Program"),
        _ => None,
    }
}

// Комиссия: базовая часть и приоритетная часть
struct FeeBreakdown {
    base: u64,
    priority: u64,
}

impl FeeBreakdown {
    fn total(&self) -> u64 {
        self.base + self.priority
    }
}

// Приоритетная часть = лимит CU * цена CU / 1 000 000, с округлением вверх
fn calculate_fee(signatures: u64, cu_limit: u64, micro_lamports_per_cu: u64) -> FeeBreakdown {
    let base = signatures * 5_000;
    let product = (cu_limit as u128) * (micro_lamports_per_cu as u128);
    let priority = ((product + 999_999) / 1_000_000) as u64;
    FeeBreakdown { base, priority }
}

// Переводит lamports в текст SOL с 9 знаками после точки
fn lamports_to_sol(lamports: u64) -> String {
    format!("{}.{:09}", lamports / 1_000_000_000, lamports % 1_000_000_000)
}

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