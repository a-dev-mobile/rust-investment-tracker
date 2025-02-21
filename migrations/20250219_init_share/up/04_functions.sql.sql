-- Create function for updating shares
CREATE OR REPLACE FUNCTION instrument_services.update_shares(shares_json jsonb) 
RETURNS integer
LANGUAGE plpgsql
AS $$
DECLARE
    affected_rows INTEGER;
BEGIN
    -- Create temporary table for new data
    CREATE TEMPORARY TABLE temp_shares (
        figi VARCHAR(20) NOT NULL,
        ticker VARCHAR(20) NOT NULL,
        class_code VARCHAR(50) NOT NULL,
        isin VARCHAR(20) NOT NULL,
        uid VARCHAR(36) NOT NULL,
        position_uid VARCHAR(36) NOT NULL,
        name VARCHAR(200) NOT NULL,
        lot INTEGER NOT NULL,
        currency VARCHAR(10) NOT NULL,
        exchange VARCHAR(50) NOT NULL,
        klong instrument_services.quotation,
        kshort instrument_services.quotation,
        dlong instrument_services.quotation,
        dshort instrument_services.quotation,
        dlong_min instrument_services.quotation,
        dshort_min instrument_services.quotation,
        short_enabled_flag BOOLEAN NOT NULL,
        trading_status instrument_services.security_trading_status NOT NULL,
        otc_flag BOOLEAN NOT NULL,
        buy_available_flag BOOLEAN NOT NULL,
        sell_available_flag BOOLEAN NOT NULL,
        div_yield_flag BOOLEAN NOT NULL,
        share_type instrument_services.share_type NOT NULL,
        min_price_increment instrument_services.quotation,
        api_trade_available_flag BOOLEAN NOT NULL,
        real_exchange instrument_services.real_exchange NOT NULL,
        ipo_date TIMESTAMP WITH TIME ZONE,
        issue_size BIGINT,
        issue_size_plan BIGINT,
        nominal instrument_services.money_value,
        country_of_risk VARCHAR(2) NOT NULL,
        country_of_risk_name VARCHAR(100) NOT NULL,
        sector VARCHAR(200) NOT NULL,
        for_iis_flag BOOLEAN NOT NULL,
        for_qual_investor_flag BOOLEAN NOT NULL,
        weekend_flag BOOLEAN NOT NULL,
        blocked_tca_flag BOOLEAN NOT NULL,
        liquidity_flag BOOLEAN NOT NULL,
        first_1min_candle_date TIMESTAMP WITH TIME ZONE,
        first_1day_candle_date TIMESTAMP WITH TIME ZONE
    ) ON COMMIT DROP;

    -- Insert data from JSON into temporary table
    INSERT INTO temp_shares
    SELECT 
        (value->>'figi')::VARCHAR,
        (value->>'ticker')::VARCHAR,
        (value->>'class_code')::VARCHAR,
        (value->>'isin')::VARCHAR,
        (value->>'uid')::VARCHAR,
        (value->>'position_uid')::VARCHAR,
        (value->>'name')::VARCHAR,
        (value->>'lot')::INTEGER,
        (value->>'currency')::VARCHAR,
        (value->>'exchange')::VARCHAR,
        (ROW((value->'klong'->>'units')::BIGINT, (value->'klong'->>'nano')::INTEGER))::instrument_services.quotation,
        (ROW((value->'kshort'->>'units')::BIGINT, (value->'kshort'->>'nano')::INTEGER))::instrument_services.quotation,
        (ROW((value->'dlong'->>'units')::BIGINT, (value->'dlong'->>'nano')::INTEGER))::instrument_services.quotation,
        (ROW((value->'dshort'->>'units')::BIGINT, (value->'dshort'->>'nano')::INTEGER))::instrument_services.quotation,
        (ROW((value->'dlong_min'->>'units')::BIGINT, (value->'dlong_min'->>'nano')::INTEGER))::instrument_services.quotation,
        (ROW((value->'dshort_min'->>'units')::BIGINT, (value->'dshort_min'->>'nano')::INTEGER))::instrument_services.quotation,
        (value->>'short_enabled_flag')::BOOLEAN,
        (value->>'trading_status')::instrument_services.security_trading_status,
        (value->>'otc_flag')::BOOLEAN,
        (value->>'buy_available_flag')::BOOLEAN,
        (value->>'sell_available_flag')::BOOLEAN,
        (value->>'div_yield_flag')::BOOLEAN,
        (value->>'share_type')::instrument_services.share_type,
        (ROW((value->'min_price_increment'->>'units')::BIGINT, (value->'min_price_increment'->>'nano')::INTEGER))::instrument_services.quotation,
        (value->>'api_trade_available_flag')::BOOLEAN,
        (value->>'real_exchange')::instrument_services.real_exchange,
        (value->>'ipo_date')::TIMESTAMP WITH TIME ZONE,
        (value->>'issue_size')::BIGINT,
        (value->>'issue_size_plan')::BIGINT,
        (ROW(
            (value->'nominal'->>'currency')::VARCHAR,
            (value->'nominal'->>'units')::BIGINT,
            (value->'nominal'->>'nano')::INTEGER
        ))::instrument_services.money_value,
        (value->>'country_of_risk')::VARCHAR,
        (value->>'country_of_risk_name')::VARCHAR,
        (value->>'sector')::VARCHAR,
        (value->>'for_iis_flag')::BOOLEAN,
        (value->>'for_qual_investor_flag')::BOOLEAN,
        (value->>'weekend_flag')::BOOLEAN,
        (value->>'blocked_tca_flag')::BOOLEAN,
        (value->>'liquidity_flag')::BOOLEAN,
        (value->>'first_1min_candle_date')::TIMESTAMP WITH TIME ZONE,
        (value->>'first_1day_candle_date')::TIMESTAMP WITH TIME ZONE
    FROM jsonb_array_elements(shares_json);

    -- Begin atomic update
    BEGIN
        -- Delete all existing records
        DELETE FROM instrument_services.share;
        
        -- Insert new records with explicit column list
        INSERT INTO instrument_services.share (
            figi, ticker, class_code, isin, uid, position_uid, name, lot, currency, 
            exchange, klong, kshort, dlong, dshort, dlong_min, dshort_min, 
            short_enabled_flag, trading_status, otc_flag, buy_available_flag, 
            sell_available_flag, div_yield_flag, share_type, min_price_increment, 
            api_trade_available_flag, real_exchange, ipo_date, issue_size, 
            issue_size_plan, nominal, country_of_risk, country_of_risk_name, 
            sector, for_iis_flag, for_qual_investor_flag, weekend_flag, 
            blocked_tca_flag, liquidity_flag, first_1min_candle_date, 
            first_1day_candle_date
        )
        SELECT 
            figi, ticker, class_code, isin, uid, position_uid, name, lot, currency, 
            exchange, klong, kshort, dlong, dshort, dlong_min, dshort_min, 
            short_enabled_flag, trading_status, otc_flag, buy_available_flag, 
            sell_available_flag, div_yield_flag, share_type, min_price_increment, 
            api_trade_available_flag, real_exchange, ipo_date, issue_size, 
            issue_size_plan, nominal, country_of_risk, country_of_risk_name, 
            sector, for_iis_flag, for_qual_investor_flag, weekend_flag, 
            blocked_tca_flag, liquidity_flag, first_1min_candle_date, 
            first_1day_candle_date
        FROM temp_shares;

        GET DIAGNOSTICS affected_rows = ROW_COUNT;
    END;

    RETURN affected_rows;
END;
$$;

-- Set owner
ALTER FUNCTION instrument_services.update_shares(jsonb) OWNER TO postgres;