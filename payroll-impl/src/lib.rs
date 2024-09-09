pub mod classification {
    use chrono::NaiveDate;
    use std::any::Any;

    use payroll_domain::{PayCheck, PaymentClassification};

    #[derive(Debug, Clone, PartialEq)]
    pub struct SalariedClassification {
        salary: f32,
    }
    impl PaymentClassification for SalariedClassification {
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_pay(&self, _pc: &PayCheck) -> f32 {
            self.salary
        }
    }
    impl SalariedClassification {
        pub fn new(salary: f32) -> Self {
            Self { salary }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct HourlyClassification {
        hourly_rate: f32,
        timecards: Vec<TimeCard>,
    }
    impl PaymentClassification for HourlyClassification {
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_pay(&self, pc: &PayCheck) -> f32 {
            let pay_period = pc.get_pay_period();
            let mut total_pay = 0.0;
            for tc in self.timecards.iter() {
                if pay_period.contains(&tc.get_date()) {
                    total_pay += self.calculate_pay_for_timecard(tc);
                }
            }
            total_pay
        }
    }
    impl HourlyClassification {
        pub fn new(hourly_rate: f32) -> Self {
            Self {
                hourly_rate,
                timecards: vec![],
            }
        }
        pub fn add_timecard(&mut self, tc: TimeCard) {
            self.timecards.push(tc);
        }
        pub fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f32 {
            let hours = tc.get_hours();
            let overtime = (hours - 8.0).max(0.0);
            let straight_time = hours - overtime;
            straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct CommissionedClassification {
        salary: f32,
        commission_rate: f32,
        sales_receipts: Vec<SalesReceipt>,
    }
    impl PaymentClassification for CommissionedClassification {
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_pay(&self, pc: &PayCheck) -> f32 {
            let mut total_pay = self.salary;
            let pay_period = pc.get_pay_period();
            for sr in self.sales_receipts.iter() {
                if pay_period.contains(&sr.get_date()) {
                    total_pay += self.calculate_pay_for_sales_receipt(sr);
                }
            }
            total_pay
        }
    }
    impl CommissionedClassification {
        pub fn new(salary: f32, commission_rate: f32) -> Self {
            Self {
                salary,
                commission_rate,
                sales_receipts: vec![],
            }
        }
        pub fn add_sales_receipt(&mut self, sr: SalesReceipt) {
            self.sales_receipts.push(sr);
        }
        pub fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f32 {
            self.commission_rate * sr.get_amount()
        }
    }

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

    #[derive(Debug, Clone, PartialEq)]
    pub struct SalesReceipt {
        date: NaiveDate,
        amount: f32,
    }
    impl SalesReceipt {
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
}

pub mod schedule {
    use chrono::{Datelike, Days, NaiveDate, Weekday};
    use std::ops::RangeInclusive;

    use payroll_domain::PaymentSchedule;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MonthlySchedule;
    impl PaymentSchedule for MonthlySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            self.is_last_day_of_month(date)
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            // pay_date should be last_day of month
            pay_date.with_day(1).unwrap()..=pay_date
        }
    }
    impl MonthlySchedule {
        pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
            date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct WeeklySchedule;
    impl PaymentSchedule for WeeklySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            date.weekday() == Weekday::Fri
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct BiweeklySchedule;
    impl PaymentSchedule for BiweeklySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
        }
    }
}

pub mod method {
    use payroll_domain::{PayCheck, PaymentMethod};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct HoldMethod;
    impl PaymentMethod for HoldMethod {
        fn pay(&self, pc: &PayCheck) {
            // concrete implementation
            println!("HoldMethod: {:#?}", pc);
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MailMethod {
        address: String,
    }
    impl PaymentMethod for MailMethod {
        fn pay(&self, pc: &PayCheck) {
            // concrete implementation
            println!("MailMethod for {}: {:#?}", self.address, pc);
        }
    }
    impl MailMethod {
        pub fn new(address: String) -> Self {
            Self { address }
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct DirectMethod {
        bank: String,
        account: String,
    }
    impl PaymentMethod for DirectMethod {
        fn pay(&self, pc: &PayCheck) {
            // concrete implementation
            println!("DirectMethod to {}{}: {:#?}", self.bank, self.account, pc);
        }
    }
    impl DirectMethod {
        pub fn new(bank: String, account: String) -> Self {
            Self { bank, account }
        }
    }
}

pub mod affiliation {
    use chrono::{Datelike, NaiveDate, Weekday};
    use std::any::Any;

    use payroll_domain::{Affiliation, MemberId, PayCheck};

    #[derive(Debug, Clone, PartialEq)]
    pub struct UnionAffiliation {
        member_id: MemberId,
        dues: f32,

        service_charges: Vec<ServiceCharge>,
    }
    impl UnionAffiliation {
        pub fn new(member_id: MemberId, dues: f32) -> Self {
            Self {
                member_id,
                dues,
                service_charges: vec![],
            }
        }
        pub fn get_member_id(&self) -> MemberId {
            self.member_id
        }
        pub fn get_dues(&self) -> f32 {
            self.dues
        }
        pub fn add_service_charge(&mut self, sc: ServiceCharge) {
            self.service_charges.push(sc);
        }
    }
    impl Affiliation for UnionAffiliation {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_deductions(&self, pc: &PayCheck) -> f32 {
            let mut total_deductions = 0.0;
            let pay_period = pc.get_pay_period();
            for d in pc.get_pay_period().start().iter_days() {
                if d > *pay_period.end() {
                    break;
                }
                if d.weekday() == Weekday::Fri {
                    total_deductions += self.get_dues();
                }
            }
            for sc in self.service_charges.iter() {
                if pay_period.contains(&sc.get_date()) {
                    total_deductions += sc.get_amount();
                }
            }
            total_deductions
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct NoAffiliation;
    impl Affiliation for NoAffiliation {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

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
}
