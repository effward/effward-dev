use std::str::FromStr;

use super::ServerError;

#[derive(Clone, Debug, PartialEq)]
pub enum Environment {
    Production,
    Development,
}

impl FromStr for Environment {
    type Err = ServerError;

    fn from_str(input: &str) -> Result<Environment, Self::Err> {
        let input_lower: &str = &input.to_lowercase();
        match input_lower {
            "development" | "dev" => Ok(Environment::Development),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(ServerError::Environment),
        }
    }
}
