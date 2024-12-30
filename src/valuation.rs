use crate::contract::Contract;
use crate::currency::Currency;
use crate::observable::Observable;
use chrono::NaiveDate;
use std::collections::HashMap;

pub type RandomVariable = f64;
pub type ValueProcess = HashMap<NaiveDate, RandomVariable>;

pub fn exch(home_currency: Currency, foreign_currency: Currency) -> ValueProcess {
    todo!()
}

fn disc(quantity: RandomVariable, date: NaiveDate) -> ValueProcess {
    todo!()
}

fn eval(contract: &Contract, evaluation_date: NaiveDate) -> ValueProcess {
    match contract {
        Contract::Zero => [(evaluation_date, 0.0)].into_iter().collect(),
        // TODO: This is bugged, it should use exchange rates
        Contract::One(_) => [(evaluation_date, 1.0)].into_iter().collect(),
        Contract::Give(sub) => eval(sub, evaluation_date)
            .into_iter()
            .map(|(d, v)| (d, -v))
            .collect(),
        // To test "And" one can define two contracts with different maturities and check that the
        // value of the combined contract is the sum of the values if they are alive while it is
        // the value of the alive contract if only one is alive
        Contract::And(left, right) => {
            let left_value = eval(left, evaluation_date);
            let right_value = eval(right, evaluation_date);
            left_value
                .into_iter()
                .chain(right_value)
                .fold(HashMap::new(), |mut acc, (d, v)| {
                    *acc.entry(d).or_insert(0.0) += v;
                    acc
                })
        }
        // Same as "And" but with the maximum value
        Contract::Or(left, right) => {
            let left_value = eval(left, evaluation_date);
            let right_value = eval(right, evaluation_date);
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
        Contract::Scale(observable, sub) => {
            let scale = match observable {
                Observable::Constant(v) => *v,
                _ => 1.0, // Only handle Constant observables for scaling.
            };
            eval(sub, evaluation_date)
                .into_iter()
                .map(|(d, v)| (d, v * scale))
                .collect()
        }
        // To test "Truncate" one can define a contract with a maturity date and check that the
        // value is zero after the maturity date. Another test is to define a contract without a
        // maturity date, check that the value is not zero and when truncating the contract the
        // value is zero.
        Contract::Truncate(expiry, sub) => {
            if evaluation_date <= *expiry {
                eval(sub, evaluation_date)
            } else {
                [(evaluation_date, 0.0)].into_iter().collect()
            }
        }
        // To test "Then" one can define two contracts with different maturities and check that the
        // value is the value of the first contract if the date is before the maturity date of
        // itself while is the value of the second one if the date is after that maturity
        Contract::Then(first, second) => {
            let first_value = eval(first, evaluation_date);
            if let Some(expiry) = first_value.keys().max() {
                if evaluation_date > *expiry {
                    let second_value = eval(second, evaluation_date);
                    first_value.into_iter().chain(second_value).collect()
                } else {
                    first_value
                }
            } else {
                first_value
            }
        }
        // TODO: test for get is the most important one
        Contract::Get(sub) => {
            let sub_value = eval(sub, evaluation_date);
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

        Contract::AnyTime(_sub) => unimplemented!("Not implemented"),
    }
}
