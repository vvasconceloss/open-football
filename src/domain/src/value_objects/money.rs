use crate::errors::DomainError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Currency {
    EUR,
    GBP,
    USD,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money {
    amount: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn add(self, other: Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {}

        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }

    pub fn subtract(self, other: Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {}

        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency,
        })
    }

    pub fn multiply(self, factor: f64) -> Self {
        let result = (self.amount as f64 * factor).round() as i64;

        Self {
            amount: result,
            currency: self.currency,
        }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: 0,
            currency,
        }
    }
}
