use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub enum Currency {
    Usd,
    Gbp,
    Eur,
}

#[derive(Debug)]
pub struct Time(pub NaiveDate);

#[derive(Debug)]
pub enum Observable {
    Constant(f64),
    ExchangeRate(Currency, Currency),
}

#[derive(Debug)]
pub enum Combinator {
    Zero,
    One(Currency),
    Give(Rc<Combinator>),
    And(Rc<Combinator>, Rc<Combinator>),
    Or(Rc<Combinator>, Rc<Combinator>),
    Scale(Observable, Rc<Combinator>),
    Truncate(Time, Rc<Combinator>),
    Then(Rc<Combinator>, Rc<Combinator>),
}

#[derive(Debug)]
pub struct Contract(pub Combinator);

#[derive(Debug)]
pub struct ContractBuilder(Combinator);

impl Default for ContractBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractBuilder {
    pub fn new() -> Self {
        ContractBuilder(Combinator::Zero)
    }

    pub fn one(self, currency: Currency) -> Self {
        ContractBuilder(Combinator::One(currency))
    }

    pub fn scale(self, observable: Observable) -> Self {
        ContractBuilder(Combinator::Scale(observable, Rc::from(self.0)))
    }

    pub fn give(self) -> Self {
        ContractBuilder(Combinator::Give(Rc::from(self.0)))
    }

    pub fn and(self, other: ContractBuilder) -> Self {
        ContractBuilder(Combinator::And(Rc::from(self.0), Rc::from(other.0)))
    }

    pub fn or(self, other: ContractBuilder) -> Self {
        ContractBuilder(Combinator::Or(Rc::from(self.0), Rc::from(other.0)))
    }

    pub fn truncate(self, date: Time) -> Self {
        ContractBuilder(Combinator::Truncate(date, Rc::from(self.0)))
    }

    pub fn build(self) -> Contract {
        Contract(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero_coupon_bond() {
        let maturity_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        let zero_coupon_bond = ContractBuilder::new()
            .one(Currency::Usd)
            .scale(Observable::Constant(100.0))
            .truncate(Time(maturity_date))
            .build();
        println!("{:?}", zero_coupon_bond);
    }
}
