use std::{cell::RefCell, collections::HashMap};

use thiserror::Error;
use tx_rs::Tx;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum EmployeeDaoError {
    #[error("insert error: {0}")]
    InsertError(String),
}
trait EmployeeDao<Ctx> {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
}
trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>>;
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum EmployeeUsecaseError {
    #[error("entry employee failed: {0}")]
    EntryEmployeeFailed(EmployeeDaoError),
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
            .map_err(EmployeeUsecaseError::EntryEmployeeFailed)
    }
}

trait PaymentClassification {}
#[derive(Debug, Clone, PartialEq)]
struct SalariedClassification(f64);
impl PaymentClassification for SalariedClassification {}

trait PaymentSchedule {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct MonthlySchedule;
impl PaymentSchedule for MonthlySchedule {}

trait PaymentMethod {}
#[derive(Debug, Clone, Eq, PartialEq)]
struct HoldMethod;
impl PaymentMethod for HoldMethod {}

type EmployeeId = u32;
struct Employee {
    emp_id: EmployeeId,
    name: String,
    address: String,
    classification: Box<dyn PaymentClassification>,
    schedule: Box<dyn PaymentSchedule>,
    method: Box<dyn PaymentMethod>,
}

struct AddSalariedEmployeeTransaction {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f64,
}

struct MockDb {
    employee: RefCell<HashMap<EmployeeId, Employee>>,
}
impl EmployeeDao<()> for MockDb {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            let emp_id = emp.emp_id;
            self.employee.borrow_mut().insert(emp_id, emp);
            Ok(emp_id)
        })
    }
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
        Box::new(SalariedClassification(self.salary))
    }
    fn get_schedule(&self) -> Box<dyn PaymentSchedule> {
        Box::new(MonthlySchedule)
    }
}

fn main() {
    let db = MockDb {
        employee: RefCell::new(HashMap::new()),
    };

    let req = AddSalariedEmployeeTransaction {
        db,
        emp_id: 1,
        name: "Bob".to_string(),
        address: "Home".to_string(),
        salary: 1000.00,
    };
    let emp_id = req.execute().run(&mut ());
    println!("emp_id: {:?}", emp_id);
    let binding = req.db.employee.borrow();
    let emp = binding.get(&1).unwrap();
    println!(
        "id={}, name='{}', address='{}'",
        emp.emp_id, emp.name, emp.address
    );
}
