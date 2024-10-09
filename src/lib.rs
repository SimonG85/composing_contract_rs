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
pub struct Contract(pub Rc<Combinator>);

impl Contract {
    pub fn evaluate(&self, date: &NaiveDate) -> f64 {
        self.0.evaluate(date)
    }
}

#[derive(Debug)]
pub struct ContractBuilder(Rc<Combinator>);

impl Default for ContractBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractBuilder {
    pub fn new() -> Self {
        ContractBuilder(Rc::new(Combinator::Zero))
    }

    pub fn one(self, currency: Currency) -> Self {
        ContractBuilder(Rc::new(Combinator::One(currency)))
    }

    pub fn scale(self, observable: Observable) -> Self {
        ContractBuilder(Rc::new(Combinator::Scale(observable, Rc::clone(&self.0))))
    }

    pub fn give(self) -> Self {
        ContractBuilder(Rc::new(Combinator::Give(Rc::clone(&self.0))))
    }

    pub fn and(self, other: ContractBuilder) -> Self {
        ContractBuilder(Rc::new(Combinator::And(
            Rc::clone(&self.0),
            Rc::clone(&other.0),
        )))
    }

    pub fn or(self, other: ContractBuilder) -> Self {
        ContractBuilder(Rc::new(Combinator::Or(
            Rc::clone(&self.0),
            Rc::clone(&other.0),
        )))
    }

    pub fn truncate(self, date: Time) -> Self {
        ContractBuilder(Rc::new(Combinator::Truncate(date, Rc::clone(&self.0))))
    }

    pub fn then(self, other: ContractBuilder) -> Self {
        ContractBuilder(Rc::new(Combinator::Then(
            Rc::clone(&self.0),
            Rc::clone(&other.0),
        )))
    }

    pub fn build(self) -> Contract {
        Contract(self.0)
    }
}

impl Combinator {
    pub fn evaluate(&self, date: &NaiveDate) -> f64 {
        match self {
            Combinator::Zero => 0.0,
            Combinator::One(_) => 1.0,
            Combinator::Give(inner) => -inner.evaluate(date),
            Combinator::And(lhs, rhs) => lhs.evaluate(date) + rhs.evaluate(date),
            Combinator::Or(lhs, rhs) => lhs.evaluate(date).max(rhs.evaluate(date)),
            Combinator::Scale(Observable::Constant(factor), inner) => factor * inner.evaluate(date),
            Combinator::Truncate(Time(maturity), inner) => {
                if date <= maturity {
                    inner.evaluate(date)
                } else {
                    0.0
                }
            }
            Combinator::Then(lhs, rhs) => {
                if lhs.evaluate(date) != 0.0 {
                    lhs.evaluate(date)
                } else {
                    rhs.evaluate(date)
                }
            }
            _ => unimplemented!("Implementation not available for this combinator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero_coupon_bond_at_maturity() {
        let maturity_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        let zero_coupon_bond = ContractBuilder::new()
            .one(Currency::Usd)
            .scale(Observable::Constant(100.0))
            .truncate(Time(maturity_date))
            .build();

        assert_eq!(zero_coupon_bond.evaluate(&maturity_date), 100.0);
    }
    #[test]
    fn test_zero_coupon_bond_after_maturity() {
        let maturity_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        let evaluation_date = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();
        let zero_coupon_bond = ContractBuilder::new()
            .one(Currency::Usd)
            .scale(Observable::Constant(100.0))
            .truncate(Time(maturity_date))
            .build();

        assert_eq!(zero_coupon_bond.evaluate(&evaluation_date), 0.0);
    }
}
