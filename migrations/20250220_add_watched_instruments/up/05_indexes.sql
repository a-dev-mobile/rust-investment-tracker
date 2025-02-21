-- Create indexes for watched_instruments table
CREATE INDEX idx_watched_instruments_active ON instrument_services.watched_instruments(is_active);
CREATE INDEX idx_watched_instruments_interval ON instrument_services.watched_instruments(subscription_interval);
CREATE INDEX idx_watched_instruments_uid ON instrument_services.watched_instruments(uid);
CREATE INDEX idx_watched_instruments_figi ON instrument_services.watched_instruments(figi);