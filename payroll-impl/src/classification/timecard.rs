use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeCard {
    date: NaiveDate,
    hours: f32,
}
impl TimeCard {
    pub fn new(date: NaiveDate, hours: f32) -> Self {
        Self { date, hours }
    }
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }
    pub fn get_hours(&self) -> f32 {
        self.hours
    }
}
