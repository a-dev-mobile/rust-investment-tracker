-- Create indexes for efficient querying
CREATE INDEX idx_candles_figi ON instrument_services.candles(figi);
CREATE INDEX idx_candles_instrument_uid ON instrument_services.candles(instrument_uid);
CREATE INDEX idx_candles_time ON instrument_services.candles(time);
CREATE INDEX idx_candles_interval_id ON instrument_services.candles(interval_id);
CREATE INDEX idx_candles_figi_time ON instrument_services.candles(figi, time);

-- Create index for time-based queries within a specific instrument
CREATE INDEX idx_candles_composite ON instrument_services.candles(figi, interval_id, time DESC);