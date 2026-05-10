use bigdecimal::{BigDecimal, Zero};
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone)]
pub struct AccrualConfig {
    pub annual_allowance: BigDecimal,
    pub monthly_accrual_rate: BigDecimal,
    pub max_rollover: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct StaffTenure {
    pub start_date: NaiveDate,
    pub current_date: NaiveDate,
}

/// Core domain logic for leave accrual.
/// [Ref: Raven Paper on HRIS Logic, pg. 12]
pub struct AccrualEngine;

impl AccrualEngine {
    /// Calculates the accrued leave for a staff member based on their tenure.
    /// Logic: (Full months worked * monthly_accrual_rate) - (leaves taken)
    /// Note: This function only calculates the ACCRUED amount, not the final balance.
    pub fn calculate_accrued(
        tenure: &StaffTenure,
        config: &AccrualConfig,
    ) -> BigDecimal {
        if tenure.current_date <= tenure.start_date {
            return BigDecimal::zero();
        }

        let years = tenure.current_date.year() - tenure.start_date.year();
        let months = tenure.current_date.month() as i32 - tenure.start_date.month() as i32;
        let total_months = (years * 12) + months;

        if total_months <= 0 {
            return BigDecimal::zero();
        }

        let accrued = config.monthly_accrual_rate.clone() * BigDecimal::from(total_months);
        
        // Cap by annual allowance if applicable (simple rule for now)
        if accrued > config.annual_allowance {
            config.annual_allowance.clone()
        } else {
            accrued
        }
    }

    /// Calculates the balance after rollover at the end of a year.
    /// Logic: min(current_balance, max_rollover)
    pub fn calculate_rollover(
        current_balance: &BigDecimal,
        config: &AccrualConfig,
    ) -> BigDecimal {
        if current_balance > &config.max_rollover {
            config.max_rollover.clone()
        } else {
            current_balance.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_basic_accrual() {
        let config = AccrualConfig {
            annual_allowance: BigDecimal::from(15),
            monthly_accrual_rate: BigDecimal::from_str("1.25").unwrap(),
            max_rollover: BigDecimal::from(5),
        };

        let tenure = StaffTenure {
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            current_date: NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(), // 4 months
        };

        let result = AccrualEngine::calculate_accrued(&tenure, &config);
        assert_eq!(result.to_string(), "5.00");
    }

    #[test]
    fn test_zero_accrual_for_new_joiner() {
        let config = AccrualConfig {
            annual_allowance: BigDecimal::from(15),
            monthly_accrual_rate: BigDecimal::from_str("1.25").unwrap(),
            max_rollover: BigDecimal::from(5),
        };

        let tenure = StaffTenure {
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            current_date: NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
        };

        let result = AccrualEngine::calculate_accrued(&tenure, &config);
        assert_eq!(result, BigDecimal::zero());
    }

    #[test]
    fn test_rollover() {
        let config = AccrualConfig {
            annual_allowance: BigDecimal::from(15),
            monthly_accrual_rate: BigDecimal::from_str("1.25").unwrap(),
            max_rollover: BigDecimal::from(5),
        };

        let current_balance = BigDecimal::from(8);
        let rolled_over = AccrualEngine::calculate_rollover(&current_balance, &config);
        assert_eq!(rolled_over.to_string(), "5");

        let small_balance = BigDecimal::from(3);
        let rolled_over_small = AccrualEngine::calculate_rollover(&small_balance, &config);
        assert_eq!(rolled_over_small.to_string(), "3");
    }
}
