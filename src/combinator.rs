use crate::currency::Currency;
use crate::observable::Observable;
use chrono::NaiveDate;
use std::rc::Rc;

#[derive(Debug)]
pub enum Combinator {
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
