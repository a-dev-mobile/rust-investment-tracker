-- Создание схемы для данных Tinkoff Инвестиций
CREATE SCHEMA IF NOT EXISTS tinkoff_investments;

COMMENT ON SCHEMA tinkoff_investments IS 'Схема для хранения данных из Tinkoff Инвестиций API';

-- Создание таблицы акций (shares)
CREATE TABLE tinkoff_investments.shares (
    -- Идентификаторы инструмента
    figi VARCHAR(20) NOT NULL,                      -- Figi-идентификатор инструмента
    ticker VARCHAR(20) NOT NULL,                    -- Тикер инструмента
    class_code VARCHAR(50) NOT NULL,                -- Класс-код (секция торгов)
    isin VARCHAR(20) NOT NULL,                      -- Isin-идентификатор инструмента
    uid VARCHAR(36) NOT NULL PRIMARY KEY,           -- Уникальный идентификатор инструмента
    position_uid VARCHAR(36) NOT NULL,              -- Уникальный идентификатор позиции инструмента
    
    -- Основная информация
    name VARCHAR(200) NOT NULL,                     -- Название инструмента
    lot INTEGER NOT NULL,                           -- Лотность инструмента
    currency VARCHAR(10) NOT NULL,                  -- Валюта расчётов
    exchange VARCHAR(50) NOT NULL,                  -- Tорговая площадка (секция биржи)
    
    -- Параметры риска - разбиваем на отдельные поля
    klong_units BIGINT,                             -- Коэффициент ставки риска длинной позиции (целая часть)
    klong_nano INTEGER,                             -- Коэффициент ставки риска длинной позиции (дробная часть в нано)
    
    kshort_units BIGINT,                            -- Коэффициент ставки риска короткой позиции (целая часть)
    kshort_nano INTEGER,                            -- Коэффициент ставки риска короткой позиции (дробная часть в нано)
    
    dlong_units BIGINT,                             -- Ставка риска начальной маржи для КСУР лонг (целая часть)
    dlong_nano INTEGER,                             -- Ставка риска начальной маржи для КСУР лонг (дробная часть в нано)
    
    dshort_units BIGINT,                            -- Ставка риска начальной маржи для КСУР шорт (целая часть)
    dshort_nano INTEGER,                            -- Ставка риска начальной маржи для КСУР шорт (дробная часть в нано)
    
    dlong_min_units BIGINT,                         -- Ставка риска начальной маржи для КПУР лонг (целая часть)
    dlong_min_nano INTEGER,                         -- Ставка риска начальной маржи для КПУР лонг (дробная часть в нано)
    
    dshort_min_units BIGINT,                        -- Ставка риска начальной маржи для КПУР шорт (целая часть)
    dshort_min_nano INTEGER,                        -- Ставка риска начальной маржи для КПУР шорт (дробная часть в нано)
    
    -- Параметры торговли
    short_enabled_flag BOOLEAN NOT NULL,            -- Признак доступности для операций в шорт
    trading_status INTEGER NOT NULL,                -- Текущий режим торгов инструмента
    otc_flag BOOLEAN NOT NULL,                      -- Признак внебиржевой ценной бумаги
    buy_available_flag BOOLEAN NOT NULL,            -- Признак доступности для покупки
    sell_available_flag BOOLEAN NOT NULL,           -- Признак доступности для продажи
    div_yield_flag BOOLEAN NOT NULL,                -- Признак наличия дивидендной доходности
    share_type INTEGER NOT NULL,                    -- Тип акции
    
    min_price_increment_units BIGINT,               -- Шаг цены (целая часть)
    min_price_increment_nano INTEGER,               -- Шаг цены (дробная часть в нано)
    
    api_trade_available_flag BOOLEAN NOT NULL,      -- Параметр указывает на возможность торговать инструментом через API
    real_exchange INTEGER NOT NULL,                 -- Реальная площадка исполнения расчётов (биржа)
    
    -- Информация о выпуске
    ipo_date TIMESTAMP WITH TIME ZONE,              -- Дата IPO акции в часовом поясе UTC
    issue_size BIGINT,                              -- Размер выпуска
    issue_size_plan BIGINT,                         -- Плановый размер выпуска
    
    nominal_currency VARCHAR(10),                   -- Валюта номинала
    nominal_units BIGINT,                           -- Номинал (целая часть)
    nominal_nano INTEGER,                           -- Номинал (дробная часть в нано)
    
    -- Информация о стране
    country_of_risk VARCHAR(2) NOT NULL,            -- Код страны риска
    country_of_risk_name VARCHAR(100) NOT NULL,     -- Наименование страны риска
    sector VARCHAR(200) NOT NULL,                   -- Сектор экономики
    
    -- Дополнительные флаги
    for_iis_flag BOOLEAN NOT NULL,                  -- Признак доступности для ИИС
    for_qual_investor_flag BOOLEAN NOT NULL,        -- Флаг отображающий доступность только для квалифицированных инвесторов
    weekend_flag BOOLEAN NOT NULL,                  -- Флаг отображающий доступность торговли инструментом по выходным
    blocked_tca_flag BOOLEAN NOT NULL,              -- Флаг заблокированного ТКС
    liquidity_flag BOOLEAN NOT NULL,                -- Флаг достаточной ликвидности
    
    -- Даты свечей
    first_1min_candle_date TIMESTAMP WITH TIME ZONE, -- Дата первой минутной свечи
    first_1day_candle_date TIMESTAMP WITH TIME ZONE -- Дата первой дневной свечи
    

);

-- Добавление комментария к таблице
COMMENT ON TABLE tinkoff_investments.shares IS 'Информация об акциях из Tinkoff Инвестиций API';

-- Создание индексов для улучшения производительности запросов
CREATE INDEX idx_shares_figi ON tinkoff_investments.shares(figi);
CREATE INDEX idx_shares_ticker ON tinkoff_investments.shares(ticker);
CREATE INDEX idx_shares_isin ON tinkoff_investments.shares(isin);
CREATE INDEX idx_shares_country ON tinkoff_investments.shares(country_of_risk);
CREATE INDEX idx_shares_sector ON tinkoff_investments.shares(sector);
