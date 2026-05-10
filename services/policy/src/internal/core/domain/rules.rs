use chrono::{NaiveDate, Datelike};
use crate::internal::core::domain::BlackoutDate;

pub struct PolicyRules;

impl PolicyRules {
    /// Validates if the leave request satisfies the minimum notice period.
    /// [Ref: HR Standard Policy, pg. 4]
    pub fn is_notice_period_valid(
        request_date: NaiveDate,
        leave_start_date: NaiveDate,
        min_notice_days: i64,
    ) -> bool {
        let diff = (leave_start_date - request_date).num_days();
        diff >= min_notice_days
    }

    /// Checks if any part of the leave period falls within a blackout date.
    pub fn is_blackout_conflict(
        leave_start: NaiveDate,
        leave_end: NaiveDate,
        blackouts: &[BlackoutDate],
    ) -> bool {
        for blackout in blackouts {
            // Overlap check: max(start1, start2) <= min(end1, end2)
            let overlap_start = leave_start.max(blackout.start_date);
            let overlap_end = leave_end.min(blackout.end_date);
            
            if overlap_start <= overlap_end {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_notice_period() {
        let req = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let start = NaiveDate::from_ymd_opt(2023, 1, 10).unwrap();
        assert!(PolicyRules::is_notice_period_valid(req, start, 7));
        assert!(!PolicyRules::is_notice_period_valid(req, start, 14));
    }

    #[test]
    fn test_blackout_conflict() {
        let blackouts = vec![
            BlackoutDate {
                id: Uuid::now_v7(),
                name: "Year End Close".to_string(),
                start_date: NaiveDate::from_ymd_opt(2023, 12, 15).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
                description: None,
                created_at: Utc::now(),
            }
        ];

        let start = NaiveDate::from_ymd_opt(2023, 12, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 20).unwrap();
        assert!(PolicyRules::is_blackout_conflict(start, end, &blackouts));

        let start_clear = NaiveDate::from_ymd_opt(2023, 11, 1).unwrap();
        let end_clear = NaiveDate::from_ymd_opt(2023, 11, 15).unwrap();
        assert!(!PolicyRules::is_blackout_conflict(start_clear, end_clear, &blackouts));
    }
}
