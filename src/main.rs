use clap::Parser;

use joint_account::{CalculationError, TransferCalculator};

fn main() -> Result<(), CalculationError> {
    let transfer_calculator = TransferCalculator::parse();

    let (first_transfer, second_transfer) = transfer_calculator.calculate()?;
    println!("{first_transfer}");
    println!("{second_transfer}");

    Ok(())
}
