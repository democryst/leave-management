-- services/staff/migrations/20260510000001_delegations.sql
CREATE TABLE delegations (
    id UUID PRIMARY KEY,
    delegator_id UUID NOT NULL REFERENCES staff(id),
    delegatee_id UUID NOT NULL REFERENCES staff(id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_delegations_delegator ON delegations(delegator_id);
CREATE INDEX idx_delegations_delegatee ON delegations(delegatee_id);
CREATE INDEX idx_delegations_active_range ON delegations(start_date, end_date) WHERE is_active = TRUE;
