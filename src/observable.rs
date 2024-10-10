use chrono::NaiveDate;
use crate::currency::Currency;

#[derive(Debug)]
pub enum Observable {
    Constant(f64),
    ExchangeRate(Currency, Currency),
    Time(NaiveDate),
}


