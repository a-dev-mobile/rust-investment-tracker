CREATE TABLE instrument_services.candles (
    -- Primary key and identifiers
    id BIGSERIAL PRIMARY KEY,
    figi VARCHAR(20) NOT NULL,
    instrument_uid VARCHAR(36) NOT NULL REFERENCES instrument_services.share(uid),
    
    -- Time information
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    last_trade_ts TIMESTAMP WITH TIME ZONE,
    interval_id INTEGER NOT NULL REFERENCES instrument_services.candle_intervals(id),
    
    -- Price information
    open instrument_services.price_value NOT NULL,
    high instrument_services.price_value NOT NULL,
    low instrument_services.price_value NOT NULL,
    close instrument_services.price_value NOT NULL,
    volume INTEGER NOT NULL,
    
    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT unique_candle UNIQUE (figi, time, interval_id)
);