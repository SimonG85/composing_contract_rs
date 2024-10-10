use crate::currency::Currency;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum Observable {
    Constant(f64),
    ExchangeRate(Currency, Currency),
    Time(NaiveDate),
}
