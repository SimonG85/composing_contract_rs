use chrono::NaiveDate;

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
