CREATE TABLE IF NOT EXISTS instrument_services.candle_intervals (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Insert the predefined intervals with explicit IDs
INSERT INTO instrument_services.candle_intervals (id, name) VALUES
    (0, 'CANDLE_INTERVAL_UNSPECIFIED'),
    (1, 'CANDLE_INTERVAL_1_MIN'),
    (2, 'CANDLE_INTERVAL_5_MIN'),
    (3, 'CANDLE_INTERVAL_15_MIN'),
    (4, 'CANDLE_INTERVAL_HOUR'),
    (5, 'CANDLE_INTERVAL_DAY'),
    (6, 'CANDLE_INTERVAL_2_MIN'),
    (7, 'CANDLE_INTERVAL_3_MIN'),
    (8, 'CANDLE_INTERVAL_10_MIN'),
    (9, 'CANDLE_INTERVAL_30_MIN'),
    (10, 'CANDLE_INTERVAL_2_HOUR'),
    (11, 'CANDLE_INTERVAL_4_HOUR'),
    (12, 'CANDLE_INTERVAL_WEEK'),
    (13, 'CANDLE_INTERVAL_MONTH');

-- Create sequence for future inserts, starting after our manual values
CREATE SEQUENCE IF NOT EXISTS instrument_services.candle_intervals_id_seq
    START WITH 14
    OWNED BY instrument_services.candle_intervals.id;

-- Create composite type for price values
CREATE TYPE instrument_services.price_value AS (
    units BIGINT,
    nano INTEGER
);
