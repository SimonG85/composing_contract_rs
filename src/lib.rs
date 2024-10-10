use chrono::NaiveDate;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum Currency {
    Usd,
    Gbp,
    Eur,
}
#[derive(Debug)]
pub enum InterestRateModel {
    Fixed(f64),
}

impl InterestRateModel {
    pub fn interest_rate(&self, date: NaiveDate) -> f64 {
        match self {
            InterestRateModel::Fixed(rate) => *rate,
        }
    }
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
        Contract(Rc::new(Combinator::Give(self.0)))
    }
    pub fn and(self, other: Contract) -> Self {
        Contract(Rc::new(Combinator::And(self.0, other.0)))
    }

    pub fn or(self, other: Contract) -> Self {
        Contract(Rc::new(Combinator::Or(self.0, other.0)))
    }
    pub fn truncate(self, date: NaiveDate) -> Self {
        Contract(Rc::new(Combinator::Truncate(date, self.0)))
    }

    pub fn then(self, other: Contract) -> Self {
        Contract(Rc::new(Combinator::Then(self.0, other.0)))
    }

    pub fn scale(self, observable: Observable) -> Self {
        Contract(Rc::new(Combinator::Scale(observable, self.0)))
    }

    pub fn get(self) -> Self {
        Contract(Rc::new(Combinator::Get(self.0)))
    }
    pub fn anytime(self) -> Self {
        Contract(Rc::new(Combinator::AnyTime(self.0)))
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

pub type RandomVariable = f64;
pub type ValueProcess = HashMap<NaiveDate, RandomVariable>;

pub fn exch(home_currency: Currency, foreign_currency: Currency) -> ValueProcess {
    todo!()
}

fn disc(quantity: RandomVariable, date: NaiveDate) -> ValueProcess {
    todo!()
}

pub fn eval(contract: &Contract, date: NaiveDate) -> ValueProcess {
    eval_combinator(&contract.0, date)
}

fn eval_combinator(combinator: &Rc<Combinator>, evaluation_date: NaiveDate) -> ValueProcess {
    match &**combinator {
        Combinator::Zero => [(evaluation_date, 0.0)].into_iter().collect(),
        // TODO: This is bugged, it should use exchange rates
        Combinator::One(_) => [(evaluation_date, 1.0)].into_iter().collect(),
        Combinator::Give(sub) => eval_combinator(sub, evaluation_date)
            .into_iter()
            .map(|(d, v)| (d, -v))
            .collect(),
        // To test "And" one can define two contracts with different maturities and check that the
        // value of the combined contract is the sum of the values if they are alive while it is
        // the value of the alive contract if only one is alive
        Combinator::And(left, right) => {
            let left_value = eval_combinator(left, evaluation_date);
            let right_value = eval_combinator(right, evaluation_date);
            left_value
                .into_iter()
                .chain(right_value)
                .fold(HashMap::new(), |mut acc, (d, v)| {
                    *acc.entry(d).or_insert(0.0) += v;
                    acc
                })
        }
        // Same as "And" but with the maximum value
        Combinator::Or(left, right) => {
            let left_value = eval_combinator(left, evaluation_date);
            let right_value = eval_combinator(right, evaluation_date);
            left_value
                .into_iter()
                .chain(right_value)
                .fold(HashMap::new(), |mut acc, (d, v)| {
                    acc.entry(d).and_modify(|e| *e = e.max(v)).or_insert(v);
                    acc
                })
        }
        // To test "Scale" one can define a contract with a constant, apply a scale to it and check
        // that the value is the product of the constant and the scale.
        Combinator::Scale(observable, sub) => {
            let scale = match observable {
                Observable::Constant(v) => *v,
                _ => 1.0, // Only handle Constant observables for scaling.
            };
            eval_combinator(sub, evaluation_date)
                .into_iter()
                .map(|(d, v)| (d, v * scale))
                .collect()
        }
        // To test "Truncate" one can define a contract with a maturity date and check that the
        // value is zero after the maturity date. Another test is to define a contract without a
        // maturity date, check that the value is not zero and when truncating the contract the
        // value is zero.
        Combinator::Truncate(expiry, sub) => {
            if evaluation_date <= *expiry {
                eval_combinator(sub, evaluation_date)
            } else {
                [(evaluation_date, 0.0)].into_iter().collect()
            }
        }
        // To test "Then" one can define two contracts with different maturities and check that the
        // value is the value of the first contract if the date is before the maturity date of
        // itself while is the value of the second one if the date is after that maturity
        Combinator::Then(first, second) => {
            let first_value = eval_combinator(first, evaluation_date);
            if let Some(expiry) = first_value.keys().max() {
                if evaluation_date > *expiry {
                    let second_value = eval_combinator(second, evaluation_date);
                    first_value.into_iter().chain(second_value).collect()
                } else {
                    first_value
                }
            } else {
                first_value
            }
        }
        // TODO: test for get is the most important one
        Combinator::Get(sub) => {
            let sub_value = eval_combinator(sub, evaluation_date);
            if let Some(expiry) = sub_value.keys().max() {
                if let Some(v) = sub_value.get(expiry) {
                    [(expiry.clone(), *v)].into_iter().collect()
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            }
        }

        Combinator::AnyTime(sub) => unimplemented!("Not implemented"),
        _ => {
            unimplemented!("Not implemented")
        }
    }
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
