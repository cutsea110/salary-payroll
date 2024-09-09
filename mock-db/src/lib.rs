use std::{cell::RefCell, collections::HashMap, rc::Rc};

use dao::{EmployeeDao, EmployeeDaoError};
use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

#[derive(Debug, Clone)]
pub struct MockDb {
    employees: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
    union_members: Rc<RefCell<HashMap<MemberId, EmployeeId>>>,
    paychecks: Rc<RefCell<HashMap<EmployeeId, Vec<Paycheck>>>>,
}
impl MockDb {
    pub fn new() -> Self {
        Self {
            employees: Rc::new(RefCell::new(HashMap::new())),
            union_members: Rc::new(RefCell::new(HashMap::new())),
            paychecks: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
impl EmployeeDao<()> for MockDb {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            let emp_id = emp.get_emp_id();
            if self.employees.borrow().contains_key(&emp_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "emp_id={} already exists",
                    emp_id
                )));
            }
            self.employees.borrow_mut().insert(emp_id, emp);
            Ok(emp_id)
        })
    }
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.employees.borrow_mut().remove(&emp_id).is_none() {
                return Err(EmployeeDaoError::DeleteError(format!(
                    "emp_id={} not found",
                    emp_id
                )));
            }
            Ok(())
        })
    }
    fn fetch(
        &self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<(), Item = Employee, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| match self.employees.borrow().get(&emp_id) {
            Some(emp) => Ok(emp.clone()),
            None => Err(EmployeeDaoError::FetchError(format!(
                "emp_id={} not found",
                emp_id
            ))),
        })
    }
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            let emp_id = emp.get_emp_id();
            if !self.employees.borrow().contains_key(&emp_id) {
                return Err(EmployeeDaoError::UpdateError(format!(
                    "emp_id={} not found",
                    emp_id
                )));
            }
            self.employees.borrow_mut().insert(emp_id, emp);
            Ok(())
        })
    }
    fn get_all(&self) -> impl tx_rs::Tx<(), Item = Vec<Employee>, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| Ok(self.employees.borrow().values().cloned().collect()))
    }

    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.union_members.borrow().contains_key(&member_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "member_id={} already exists",
                    member_id
                )));
            }
            if self.union_members.borrow().values().any(|&v| v == emp_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "emp_id={} already exists",
                    emp_id
                )));
            }
            self.union_members.borrow_mut().insert(member_id, emp_id);
            Ok(())
        })
    }
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.union_members.borrow_mut().remove(&member_id).is_none() {
                return Err(EmployeeDaoError::DeleteError(format!(
                    "member_id={} not found",
                    member_id
                )));
            }
            Ok(())
        })
    }

    fn find_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| match self.union_members.borrow().get(&member_id) {
            Some(&emp_id) => Ok(emp_id),
            None => Err(EmployeeDaoError::FetchError(format!(
                "member_id={} not found",
                member_id
            ))),
        })
    }

    fn record_paycheck(
        &self,
        emp_id: EmployeeId,
        pc: Paycheck,
    ) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            self.paychecks
                .borrow_mut()
                .entry(emp_id)
                .or_insert(vec![])
                .push(pc);
            Ok(())
        })
    }
}
