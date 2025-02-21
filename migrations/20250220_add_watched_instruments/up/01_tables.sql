CREATE TABLE IF NOT EXISTS instrument_services.subscription_intervals (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Insert the predefined intervals with explicit IDs
INSERT INTO instrument_services.subscription_intervals (id, name) VALUES
    (0, 'SUBSCRIPTION_INTERVAL_UNSPECIFIED'),
    (1, 'SUBSCRIPTION_INTERVAL_ONE_MINUTE'),
    (2, 'SUBSCRIPTION_INTERVAL_FIVE_MINUTES');

-- Create sequence for future inserts, starting after our manual values
CREATE SEQUENCE IF NOT EXISTS instrument_services.subscription_intervals_id_seq
    START WITH 3
    OWNED BY instrument_services.subscription_intervals.id;
