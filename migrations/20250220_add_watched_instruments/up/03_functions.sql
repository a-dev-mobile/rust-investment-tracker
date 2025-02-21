-- Create function to update last_updated_at timestamp
CREATE OR REPLACE FUNCTION instrument_services.update_last_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;