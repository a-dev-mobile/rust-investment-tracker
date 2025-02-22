DROP FUNCTION IF EXISTS instrument_services.update_candles CASCADE;
DROP INDEX IF EXISTS instrument_services.idx_candles_composite;
DROP INDEX IF EXISTS instrument_services.idx_candles_figi_time;
DROP INDEX IF EXISTS instrument_services.idx_candles_interval_id;
DROP INDEX IF EXISTS instrument_services.idx_candles_time;
DROP INDEX IF EXISTS instrument_services.idx_candles_instrument_uid;
DROP INDEX IF EXISTS instrument_services.idx_candles_figi;
DROP TABLE IF EXISTS instrument_services.candles CASCADE;
DROP SEQUENCE IF EXISTS instrument_services.candle_intervals_id_seq;
DROP TABLE IF EXISTS instrument_services.candle_intervals CASCADE;
DROP TYPE IF EXISTS instrument_services.price_value CASCADE;