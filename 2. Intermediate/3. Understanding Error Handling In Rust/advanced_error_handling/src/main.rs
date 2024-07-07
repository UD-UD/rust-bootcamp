#![allow(dead_code)]

use std::{collections::HashMap, io};
use std::fmt::{Debug, Display, format, Formatter, Write};
use std::num::ParseIntError;

use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{msg}")]
struct ParsePayementInfoError {
    source: Option<anyhow::Error>,
    msg: String,
}

// impl std::fmt::Debug for ParsePayementInfoError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match &self.source {
//             Some(e) => write!(f, "{:?}\n{msg}", e, msg = self.msg),
//             None => write!(f, "{msg}", msg = self.msg)
//         }
//     }
// }

// impl From<ParseIntError> for ParsePayementInfoError {
//     fn from(value: ParseIntError) -> Self {
//         ParsePayementInfoError {
//             source: Some(Box::new(value)),
//             msg: None,
//         }
//     }
// }

// impl Error for ParsePayementInfoError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.source.as_deref()
//     }
// }

// impl Display for ParsePayementInfoError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Parsing payment error: invalid payment info")
//     }
// }

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePayementInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse().with_context(|| format!("Failed to parse input as number"))
        })
        .collect::<Result<Vec<u32>, _>>().map_err(|e| ParsePayementInfoError {
        source: Some(e),
        msg: format!("Failed to parse input as number"),
    });

    numbers
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePayementInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();

    let expected_length = 4;

    if len != expected_length {
        return Err(ParsePayementInfoError {
            source: None,
            msg: format!("Incorrect Number of element passed"),
        });
    }
    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv,
    })
}

#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// impl Display for CreditCardError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Credit Card Error: Could not retrieve credit card.")
//     }
// }
//
// impl Debug for CreditCardError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
//             Self::Other(e, msg) => write!(f, "{self}\n{msg}\n{e:?}")
//         }
//     }
// }

// impl Error for CreditCardError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             CreditCardError::InvalidInput(_) => None,
//             CreditCardError::Other(err, _) => Some(err.as_ref())
//         }
//     }
// }

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}.")))?;

    let card = parse_card(card_string).with_context(|| "Credit card Cannot be passed".to_string())
        .map_err(|err| CreditCardError::Other(err));

    card
}

fn main() {
    env_logger::init();
    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "1234567 0616 123"),
        ("Bob", "1234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());
    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(e) => {
            match &e {
                CreditCardError::InvalidInput(msgTim) => println!("{}", msgTim),
                CreditCardError::Other(_) => println!("Something went wrong")
            }
            log::error!("{e:?}")
        }
    }
}