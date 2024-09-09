use abstract_tx::EmployeeUsecaseError;
use payroll_app::TestPayrollApp;
use tx_app::TransactionApplication;

fn main() -> Result<(), EmployeeUsecaseError> {
    let mut app = TestPayrollApp::new("script/test.scr");
    app.run()?;
    println!("{:#?}", app);

    Ok(())
}
