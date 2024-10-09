use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub enum Currency {
    Usd,
    Gbp,
    Eur,
}

#[derive(Debug)]
pub enum Observable {
    Constant(f64),
    ExchangeRate(Currency, Currency),
    Time(NaiveDate),
}

#[derive(Debug)]
enum Combinator {
    Zero,
    One(Currency),
    Give(Rc<Combinator>),
    And(Rc<Combinator>, Rc<Combinator>),
    Or(Rc<Combinator>, Rc<Combinator>),
    Truncate(NaiveDate, Rc<Combinator>),
    Then(Rc<Combinator>, Rc<Combinator>),
    Scale(Observable, Rc<Combinator>),
    Get(Rc<Combinator>),
    AnyTime(Rc<Combinator>),
}

#[derive(Debug)]
pub struct Contract(Rc<Combinator>);

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}

impl Contract {
    pub fn new() -> Self {
        Contract(Rc::new(Combinator::Zero))
    }

    pub fn one(currency: Currency) -> Self {
        Contract(Rc::new(Combinator::One(currency)))
    }
    pub fn give(self) -> Self {
        Contract(Rc::new(Combinator::Give(self.0.clone())))
    }
    pub fn and(self, other: Contract) -> Self {
        Contract(Rc::new(Combinator::And(self.0.clone(), other.0.clone())))
    }

    pub fn or(self, other: Contract) -> Self {
        Contract(Rc::new(Combinator::Or(self.0.clone(), other.0.clone())))
    }
    pub fn truncate(self, date: NaiveDate) -> Self {
        Contract(Rc::new(Combinator::Truncate(date, self.0.clone())))
    }

    pub fn then(self, other: Contract) -> Self {
        Contract(Rc::new(Combinator::Then(self.0.clone(), other.0.clone())))
    }

    pub fn scale(self, observable: Observable) -> Self {
        Contract(Rc::new(Combinator::Scale(observable, self.0.clone())))
    }

    pub fn get(self) -> Self {
        Contract(Rc::new(Combinator::Get(self.0.clone())))
    }
    pub fn anytime(self) -> Self {
        Contract(Rc::new(Combinator::AnyTime(self.0.clone())))
    }
}

fn zcb(maturity_date: NaiveDate, scale: f64, currency: Currency) -> Contract {
    Contract::one(currency)
        .truncate(maturity_date)
        .scale(Observable::Constant(scale))
}

fn european_option(date: NaiveDate, contract: Contract) -> Contract {
    Contract::new().or(contract).truncate(date).get()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero_coupon_bond_at_maturity() {
        let maturity_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        let zero_coupon_bond = Contract::one(Currency::Usd)
            .scale(Observable::Constant(100.0))
            .truncate(maturity_date);
    }
    #[test]
    fn test_zero_coupon_bond_after_maturity() {
        let maturity_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        let evaluation_date = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();
        let zero_coupon_bond = Contract::one(Currency::Usd)
            .scale(Observable::Constant(100.0))
            .truncate(maturity_date);
    }
}
