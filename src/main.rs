use core::fmt::Debug;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use thiserror::Error;
use tx_rs::Tx;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum EmployeeDaoError {
    #[error("insert error: {0}")]
    InsertError(String),
    #[error("delete error: {0}")]
    DeleteError(String),
}
trait EmployeeDao<Ctx> {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
}
trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>>;
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum EmployeeUsecaseError {
    #[error("register employee failed: {0}")]
    RegisterEmployeeFailed(EmployeeDaoError),
    #[error("unregister employee failed: {0}")]
    UnregisterEmployeeFailed(EmployeeDaoError),
}

trait AddEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_classification(&self) -> Box<dyn PaymentClassification>;
    fn get_schedule(&self) -> Box<dyn PaymentSchedule>;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name().to_string();
        let address = self.get_address().to_string();

        let classification = self.get_classification();
        let schedule = self.get_schedule();
        let method = Box::new(HoldMethod);
        let emp = Employee {
            emp_id,
            name,
            address,
            classification,
            schedule,
            method,
        };
        self.dao()
            .insert(emp)
            .map_err(EmployeeUsecaseError::RegisterEmployeeFailed)
    }
}

trait DeleteEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        self.dao()
            .delete(emp_id)
            .map_err(EmployeeUsecaseError::UnregisterEmployeeFailed)
    }
}

trait PaymentClassification: Debug {}
#[derive(Debug, Clone, PartialEq)]
struct SalariedClassification {
    salary: f64,
}
impl PaymentClassification for SalariedClassification {}
#[derive(Debug, Clone, PartialEq)]
struct HourlyClassification {
    hourly_rate: f64,
}
impl PaymentClassification for HourlyClassification {}
#[derive(Debug, Clone, PartialEq)]
struct CommissionedClassification {
    salary: f64,
    commission_rate: f64,
}
impl PaymentClassification for CommissionedClassification {}

trait PaymentSchedule: Debug {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct MonthlySchedule;
impl PaymentSchedule for MonthlySchedule {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct WeeklySchedule;
impl PaymentSchedule for WeeklySchedule {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct BiweeklySchedule;
impl PaymentSchedule for BiweeklySchedule {}

trait PaymentMethod: Debug {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct HoldMethod;
impl PaymentMethod for HoldMethod {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct MailMethod {
    address: String,
}
impl PaymentMethod for MailMethod {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct DirectMethod {
    bank: String,
    account: String,
}
impl PaymentMethod for DirectMethod {}

type EmployeeId = u32;
#[derive(Debug)]
struct Employee {
    emp_id: EmployeeId,
    name: String,
    address: String,
    classification: Box<dyn PaymentClassification>,
    schedule: Box<dyn PaymentSchedule>,
    method: Box<dyn PaymentMethod>,
}

#[derive(Debug, Clone)]
struct MockDb {
    employee: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
}
impl EmployeeDao<()> for MockDb {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            let emp_id = emp.emp_id;
            if self.employee.borrow().contains_key(&emp_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "emp_id={} already exists",
                    emp_id
                )));
            }
            self.employee.borrow_mut().insert(emp_id, emp);
            Ok(emp_id)
        })
    }
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.employee.borrow_mut().remove(&emp_id).is_none() {
                return Err(EmployeeDaoError::DeleteError(format!(
                    "emp_id={} not found",
                    emp_id
                )));
            }
            Ok(())
        })
    }
}

struct AddSalariedEmployeeTransaction {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f64,
}
impl HaveEmployeeDao<()> for AddSalariedEmployeeTransaction {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddEmployeeTransaction<()> for AddSalariedEmployeeTransaction {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_classification(&self) -> Box<dyn PaymentClassification> {
        Box::new(SalariedClassification {
            salary: self.salary,
        })
    }
    fn get_schedule(&self) -> Box<dyn PaymentSchedule> {
        Box::new(MonthlySchedule)
    }
}

struct AddHourlyEmployeeTransaction {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f64,
}
impl HaveEmployeeDao<()> for AddHourlyEmployeeTransaction {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddEmployeeTransaction<()> for AddHourlyEmployeeTransaction {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_classification(&self) -> Box<dyn PaymentClassification> {
        Box::new(HourlyClassification {
            hourly_rate: self.hourly_rate,
        })
    }
    fn get_schedule(&self) -> Box<dyn PaymentSchedule> {
        Box::new(WeeklySchedule)
    }
}

struct AddCommissionedEmployeeTransaction {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f64,
    commission_rate: f64,
}
impl HaveEmployeeDao<()> for AddCommissionedEmployeeTransaction {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddEmployeeTransaction<()> for AddCommissionedEmployeeTransaction {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_classification(&self) -> Box<dyn PaymentClassification> {
        Box::new(CommissionedClassification {
            salary: self.salary,
            commission_rate: self.commission_rate,
        })
    }
    fn get_schedule(&self) -> Box<dyn PaymentSchedule> {
        Box::new(BiweeklySchedule)
    }
}

struct DelEmployeeTransaction {
    db: MockDb,
    emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for DelEmployeeTransaction {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl DeleteEmployeeTransaction<()> for DelEmployeeTransaction {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}

fn main() {
    let db = MockDb {
        employee: Rc::new(RefCell::new(HashMap::new())),
    };

    let req = AddSalariedEmployeeTransaction {
        db: db.clone(),
        emp_id: 1,
        name: "Bob".to_string(),
        address: "Home".to_string(),
        salary: 1000.00,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let req = AddHourlyEmployeeTransaction {
        db: db.clone(),
        emp_id: 2,
        name: "Bill".to_string(),
        address: "Home".to_string(),
        hourly_rate: 15.25,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let req = AddCommissionedEmployeeTransaction {
        db: db.clone(),
        emp_id: 3,
        name: "Lance".to_string(),
        address: "Home".to_string(),
        salary: 2500.00,
        commission_rate: 3.2,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    for emp_id in 1..=3 {
        let req = DelEmployeeTransaction {
            db: db.clone(),
            emp_id,
        };
        let _ = req.execute().run(&mut ()).expect("delete employee");
        println!("deleted: {:#?}", db);
    }
}
