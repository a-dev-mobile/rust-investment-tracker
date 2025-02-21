-- Create indexes for share table
CREATE INDEX idx_share_figi ON instrument_services.share(figi);
CREATE INDEX idx_share_ticker ON instrument_services.share(ticker);
CREATE INDEX idx_share_isin ON instrument_services.share(isin);
