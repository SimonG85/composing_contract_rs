pub mod combinator;
pub mod contract;
pub mod currency;
pub mod interest_rate;
pub mod observable;
pub mod valuation;

use crate::contract::Contract;
use crate::currency::Currency;
use crate::observable::Observable;
use chrono::NaiveDate;

pub fn zcb(maturity_date: NaiveDate, scale: f64, currency: Currency) -> Contract {
    Contract::one(currency)
        .truncate(maturity_date)
        .scale(Observable::Constant(scale))
}

pub fn european_option(date: NaiveDate, contract: Contract) -> Contract {
    Contract::new().or(contract).truncate(date).get()
}
