use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
enum Currency {
    Usd,
    Gbp,
    Eur,
}

#[derive(Debug)]
struct Time(NaiveDate);

#[derive(Debug)]
enum Observable {
    Constant(f64),
    ExchangeRate(Currency, Currency),
}

#[derive(Debug)]
enum Combinator {
    Zero,
    One(Currency),
    Give(Rc<Combinator>),
    And(Rc<Combinator>, Rc<Combinator>),
    Or(Rc<Combinator>, Rc<Combinator>),
    Scale(Observable, Rc<Combinator>),
    Truncate {
        date: Time,
        contract: Rc<Combinator>,
    },
    Then(Rc<Combinator>, Rc<Combinator>),
}

struct Contract {
    combinator: Combinator,
}

struct ContractBuilder {
    combinator: Rc<Combinator>,
}

impl ContractBuilder {
    fn new() -> Self {
        ContractBuilder {
            combinator: Rc::new(Combinator::Zero),
        }
    }

    fn one(mut self, currency: Currency) -> Self {
        self.combinator = Rc::new(Combinator::One(currency));
        self
    }

    fn scale(mut self, observable: Observable) -> Self {
        self.combinator = Rc::new(Combinator::Scale(observable, self.combinator));
        self
    }

    fn give(mut self) -> Self {
        self.combinator = Rc::new(Combinator::Give(self.combinator));
        self
    }

    fn and(mut self, other: Rc<Combinator>) -> Self {
        self.combinator = Rc::new(Combinator::And(self.combinator, other));
        self
    }

    fn or(mut self, other: Rc<Combinator>) -> Self {
        self.combinator = Rc::new(Combinator::Or(self.combinator, other));
        self
    }

    fn truncate(mut self, date: NaiveDate) -> Self {
        self.combinator = Rc::new(Combinator::Truncate {
            date: Time(date),
            contract: self.combinator,
        });
        self
    }

    fn build(self) -> Contract {
        Contract {
            combinator: Rc::try_unwrap(self.combinator).unwrap(),
        }
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
            .truncate(maturity_date)
            .build();
    }
}
