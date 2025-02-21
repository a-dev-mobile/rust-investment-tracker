create function instrument_services.update_candles(candles_json jsonb) returns integer
    language plpgsql
as
$$
DECLARE
    affected_rows INTEGER;
BEGIN
    -- Create temporary table for new data
    CREATE TEMPORARY TABLE temp_candles (
        figi VARCHAR(20) NOT NULL,
        instrument_uid VARCHAR(36) NOT NULL,
        time TIMESTAMP WITH TIME ZONE NOT NULL,
        last_trade_ts TIMESTAMP WITH TIME ZONE,
        interval_id INTEGER NOT NULL,
        open instrument_services.price_value,
        high instrument_services.price_value,
        low instrument_services.price_value,
        close instrument_services.price_value,
        volume INTEGER NOT NULL
    ) ON COMMIT DROP;

    -- Insert data from JSON into temporary table
    INSERT INTO temp_candles
    SELECT
        (value->>'figi')::VARCHAR,
        (value->>'instrument_uid')::VARCHAR,
        (value->>'time')::TIMESTAMP WITH TIME ZONE,
        (value->>'last_trade_ts')::TIMESTAMP WITH TIME ZONE,
        (value->>'interval')::INTEGER,
        (ROW((value->'open'->>'units')::BIGINT, (value->'open'->>'nano')::INTEGER))::instrument_services.price_value,
        (ROW((value->'high'->>'units')::BIGINT, (value->'high'->>'nano')::INTEGER))::instrument_services.price_value,
        (ROW((value->'low'->>'units')::BIGINT, (value->'low'->>'nano')::INTEGER))::instrument_services.price_value,
        (ROW((value->'close'->>'units')::BIGINT, (value->'close'->>'nano')::INTEGER))::instrument_services.price_value,
        (value->>'volume')::INTEGER
    FROM jsonb_array_elements(candles_json);

    -- Insert or update records with explicit column list
    INSERT INTO instrument_services.candles (
        figi, instrument_uid, time, last_trade_ts, interval_id,
        open, high, low, close, volume
    )
    SELECT
        figi, instrument_uid, time, last_trade_ts, interval_id,
        open, high, low, close, volume
    FROM temp_candles
    ON CONFLICT (figi, time, interval_id)
    DO UPDATE SET
        last_trade_ts = EXCLUDED.last_trade_ts,
        open = EXCLUDED.open,
        high = EXCLUDED.high,
        low = EXCLUDED.low,
        close = EXCLUDED.close,
        volume = EXCLUDED.volume;

    GET DIAGNOSTICS affected_rows = ROW_COUNT;

    RETURN affected_rows;
END;
$$;

alter function update_candles(jsonb) owner to postgres;

