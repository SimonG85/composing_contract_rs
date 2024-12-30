use crate::currency::Currency;
use crate::observable::Observable;
use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub enum Contract {
    Zero,
    One(Currency),
    Give(Rc<Contract>),
    And(Rc<Contract>, Rc<Contract>),
    Or(Rc<Contract>, Rc<Contract>),
    Truncate(NaiveDate, Rc<Contract>),
    Then(Rc<Contract>, Rc<Contract>),
    Scale(Observable, Rc<Contract>),
    Get(Rc<Contract>),
    AnyTime(Rc<Contract>),
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}

impl Contract {
    pub fn new() -> Self {
        Contract::Zero
    }

    pub fn one(currency: Currency) -> Self {
        Contract::One(currency)
    }
    pub fn give(self) -> Self {
        Contract::Give(Rc::new(self))
    }
    pub fn and(self, other: Contract) -> Self {
        Contract::And(Rc::new(self), Rc::new(other))
    }

    pub fn or(self, other: Contract) -> Self {
        Contract::Or(Rc::new(self), Rc::new(other))
    }
    pub fn truncate(self, date: NaiveDate) -> Self {
        Contract::Truncate(date, Rc::new(self))
    }

    pub fn then(self, other: Contract) -> Self {
        Contract::Then(Rc::new(self), Rc::new(other))
    }

    pub fn scale(self, observable: Observable) -> Self {
        Contract::Scale(observable, Rc::new(self))
    }

    pub fn get(self) -> Self {
        Contract::Get(Rc::new(self))
    }
    pub fn anytime(self) -> Self {
        Contract::AnyTime(Rc::new(self))
    }
}
