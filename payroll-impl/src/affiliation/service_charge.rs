use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceCharge {
    date: NaiveDate,
    amount: f32,
}
impl ServiceCharge {
    pub fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
}
