-- services/policy/migrations/20260510000002_auto_approve.sql
ALTER TABLE leave_types ADD COLUMN auto_approve BOOLEAN NOT NULL DEFAULT FALSE;
