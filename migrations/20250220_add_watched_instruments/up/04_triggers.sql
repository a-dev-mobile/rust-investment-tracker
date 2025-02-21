-- Create trigger to automatically update last_updated_at
CREATE TRIGGER update_watched_instruments_timestamp
    BEFORE UPDATE ON instrument_services.watched_instruments
    FOR EACH ROW
    EXECUTE FUNCTION instrument_services.update_last_updated_at();