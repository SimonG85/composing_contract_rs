use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
enum Currency {
    Usd,
    Gbp,
}

#[derive(Debug)]
struct Time(NaiveDate);

#[derive(Debug)]
enum Observable {
    Constant(f64),
}

#[derive(Debug)]
enum Combinators {
    Zero,
    One(Currency),
    Give(Rc<Combinators>),
    And(Rc<Combinators>, Rc<Combinators>),
    Or(Rc<Combinators>, Rc<Combinators>),
    Scale(Observable, Rc<Combinators>),
    Truncate {
        date: Time,
        contract: Rc<Combinators>,
    },
    Then(Rc<Combinators>, Rc<Combinators>),
}

struct Contract {
    combinator: Combinators,
}

struct ContractBuilder {
    combinator: Rc<Combinators>,
}

impl ContractBuilder {
    fn new() -> Self {
        ContractBuilder {
            combinator: Rc::new(Combinators::Zero),
        }
    }

    fn one(mut self, currency: Currency) -> Self {
        self.combinator = Rc::new(Combinators::One(currency));
        self
    }

    fn scale(mut self, observable: Observable) -> Self {
        self.combinator = Rc::new(Combinators::Scale(observable, self.combinator));
        self
    }

    fn give(mut self) -> Self {
        self.combinator = Rc::new(Combinators::Give(self.combinator));
        self
    }

    fn and(mut self, other: Rc<Combinators>) -> Self {
        self.combinator = Rc::new(Combinators::And(self.combinator, other));
        self
    }

    fn or(mut self, other: Rc<Combinators>) -> Self {
        self.combinator = Rc::new(Combinators::Or(self.combinator, other));
        self
    }

    fn truncate(mut self, date: NaiveDate) -> Self {
        self.combinator = Rc::new(Combinators::Truncate {
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
