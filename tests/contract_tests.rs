use chrono::NaiveDate;


#[cfg(test)]
mod tests {
    use composing_contract_rs::contract::Contract;
    use composing_contract_rs::currency::Currency;
    use composing_contract_rs::observable::Observable;
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
