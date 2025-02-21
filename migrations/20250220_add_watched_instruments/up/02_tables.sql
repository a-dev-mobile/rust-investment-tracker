CREATE TABLE instrument_services.watched_instruments (
    -- Primary key
    id SERIAL PRIMARY KEY,
    
    -- Reference to share table using uid
    uid VARCHAR(36) NOT NULL REFERENCES instrument_services.share(uid),
    figi VARCHAR(20) NOT NULL,
    
    -- Watching parameters
    is_active BOOLEAN NOT NULL DEFAULT true,
    watch_started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Candle collection settings
    subscription_interval_id INTEGER NOT NULL REFERENCES instrument_services.subscription_intervals(id),
    store_history_days INTEGER NOT NULL DEFAULT 30,
    
    -- Additional parameters
    notes TEXT,
    
    -- Create unique constraint using uid instead of figi
    CONSTRAINT unique_instrument_interval UNIQUE (uid, subscription_interval_id)
);