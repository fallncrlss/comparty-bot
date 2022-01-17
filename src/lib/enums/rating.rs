use crate::lib;
use itertools::Itertools;
use std::convert::TryFrom;
use std::ops::Neg;
use std::str::FromStr;

pub enum RatingTriggers {
    Increase(Option<f64>),
    Decrease(Option<f64>),
}

impl FromStr for RatingTriggers {
    type Err = ();

    fn from_str(input: &str) -> Result<RatingTriggers, Self::Err> {
        match input {
            "+" | "спасибо" | "спс" | "благодарю" | "thanks" | "thx" | "thank you" | "👍" => {
                Ok(RatingTriggers::Increase(None))
            }
            "-" | "👎" => Ok(RatingTriggers::Decrease(None)),
            _ => {
                match input.chars().collect_vec().as_slice() {
                    ['+', amount_chars @ ..] => {
                        Ok(RatingTriggers::Increase(lib::helpers::chars_to_float(amount_chars)))
                    }
                    ['-', amount_chars @ ..] => {
                        Ok(RatingTriggers::Decrease(lib::helpers::chars_to_float(amount_chars)))
                    }
                    _ => Err(()),
                }
            },
        }
    }
}

impl RatingTriggers {
    pub fn get_sign(&self) -> char {
        match self {
            Self::Increase(_) => '+',
            Self::Decrease(_) => '-',
        }
    }

    pub fn valid_amount(&self, user_rating_amount: sqlx::types::BigDecimal) -> Result<sqlx::types::BigDecimal, String> {
        if let Some(user_rating_power) = user_rating_amount.sqrt() {
            let amount = match self {
                Self::Increase(requested_amount) | Self::Decrease(requested_amount) => {
                    match requested_amount {
                        Some(amount) => {
                            let decimal_amount = sqlx::types::BigDecimal::try_from(*amount).unwrap();
                            if user_rating_power < decimal_amount {
                                return Err(format!(
                                    "У вас недостаточное количество рейтинга для данной операции (максимум: {:.2})",
                                    user_rating_power
                                ));
                            }
                            decimal_amount
                        }
                        None => user_rating_power,
                    }
                }
            };
            return Ok(match self {
                RatingTriggers::Increase(_) => amount,
                RatingTriggers::Decrease(_) => amount.neg()
            });
        }
        Err(format!("Пользователь с негативным рейтингом не имеет право изменять чужой (рейтинг: {})", user_rating_amount))
    }
}
