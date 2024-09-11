use abstract_tx::UsecaseError;
use mock_app::TestPayrollApp;
use tx_app::TransactionApplication;

fn main() -> Result<(), UsecaseError> {
    let mut app = TestPayrollApp::new("script/test.scr");
    app.run()?;
    println!("{:#?}", app);

    Ok(())
}
