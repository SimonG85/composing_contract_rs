use crate::combinator::Combinator;
use crate::currency::Currency;
use crate::observable::Observable;
use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub struct Contract(pub(crate) Rc<Combinator>);

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
