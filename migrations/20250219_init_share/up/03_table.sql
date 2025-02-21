-- Create share table
CREATE TABLE instrument_services.share (
    -- Identifiers
    figi VARCHAR(20) NOT NULL,
    ticker VARCHAR(20) NOT NULL,
    class_code VARCHAR(50) NOT NULL,
    isin VARCHAR(20) NOT NULL,
    uid VARCHAR(36) NOT NULL PRIMARY KEY,
    position_uid VARCHAR(36) NOT NULL,
    
    -- Basic information
    name VARCHAR(200) NOT NULL,
    lot INTEGER NOT NULL,
    currency VARCHAR(10) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    
    -- Risk parameters
    klong instrument_services.quotation,
    kshort instrument_services.quotation,
    dlong instrument_services.quotation,
    dshort instrument_services.quotation,
    dlong_min instrument_services.quotation,
    dshort_min instrument_services.quotation,
    
    -- Trading parameters
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
    
    -- Issue information
    ipo_date TIMESTAMP WITH TIME ZONE,
    issue_size BIGINT,
    issue_size_plan BIGINT,
    nominal instrument_services.money_value,
    
    -- Country information
    country_of_risk VARCHAR(2) NOT NULL,
    country_of_risk_name VARCHAR(100) NOT NULL,
    sector VARCHAR(200) NOT NULL,
    
    -- Additional flags
    for_iis_flag BOOLEAN NOT NULL,
    for_qual_investor_flag BOOLEAN NOT NULL,
    weekend_flag BOOLEAN NOT NULL,
    blocked_tca_flag BOOLEAN NOT NULL,
    liquidity_flag BOOLEAN NOT NULL,
    
    -- Candle dates
    first_1min_candle_date TIMESTAMP WITH TIME ZONE,
    first_1day_candle_date TIMESTAMP WITH TIME ZONE
);