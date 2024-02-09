use std::fmt::Display;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct TransferCalculator {
    pub first_salary: f32,
    pub second_salary: f32,
    pub desired_transfer: f32,
}

impl TransferCalculator {
    pub fn new(first_salary: f32, second_salary: f32, desired_transfer: f32) -> Self {
        Self {
            first_salary,
            second_salary,
            desired_transfer,
        }
    }

    pub fn calculate(&self) -> Result<(Transfer, Transfer), CalculationError> {
        if (self.first_salary + self.second_salary) < self.desired_transfer {
            return Err(CalculationError::NotEnoughMoney);
        }

        let sum_of_salaries = self.first_salary + self.second_salary;
        let remaining_for_each = (sum_of_salaries - self.desired_transfer) / 2.0;

        let (first_transfer, second_transfer) = match (
            self.first_salary - remaining_for_each,
            self.second_salary - remaining_for_each,
        ) {
            (x, _) if x.is_sign_negative() => (0.0, self.desired_transfer),
            (_, y) if y.is_sign_negative() => (self.desired_transfer, 0.0),
            (x, y) => (x, y),
        };

        Ok((
            Transfer::new(self.first_salary, first_transfer),
            Transfer::new(self.second_salary, second_transfer),
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum CalculationError {
    NotEnoughMoney,
}

impl Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculationError::NotEnoughMoney => {
                write!(f, "Not enough money for the desired transfer in total")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Transfer {
    salary: f32,
    amount: f32,
}

impl Transfer {
    pub fn new(salary: f32, amount: f32) -> Self {
        Self { salary, amount }
    }
}

impl Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Salary {} should transfer {} and remain with {}",
            self.salary,
            self.amount,
            self.salary - self.amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_enough_money_err() {
        let tranfer_calculator = TransferCalculator::new(300.0, 500.0, 1500.0);

        assert!(tranfer_calculator.calculate().is_err());
    }

    #[test]
    fn calculates_correct_proportion() {
        let tranfer_calculator = TransferCalculator::new(1500.0, 2000.0, 1000.0);
        assert_eq!(
            tranfer_calculator.calculate(),
            Ok((Transfer::new(1500.0, 250.0), Transfer::new(2000.0, 750.0)))
        );
    }

    #[test]
    fn compensates_when_a_salary_is_less_than_the_desired_transfer() {
        let tranfer_calculator = TransferCalculator::new(1500.0, 500.0, 1000.0);
        assert_eq!(
            tranfer_calculator.calculate(),
            Ok((Transfer::new(1500.0, 1000.0), Transfer::new(500.0, 0.0)))
        );

        let tranfer_calculator = TransferCalculator::new(500.0, 1500.0, 1500.0);
        assert_eq!(
            tranfer_calculator.calculate(),
            Ok((Transfer::new(500.0, 0.0), Transfer::new(1500.0, 1000.0)))
        );
    }
}
