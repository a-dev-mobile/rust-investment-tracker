-- Скрипт для отката изменений (down migration)

-- Удаление триггера
DROP TRIGGER IF EXISTS update_shares_modtime ON tinkoff_investments.shares;

-- Удаление функции обновления времени изменения
DROP FUNCTION IF EXISTS tinkoff_investments.update_modified_column();

-- Удаление индексов
DROP INDEX IF EXISTS tinkoff_investments.idx_shares_figi;
DROP INDEX IF EXISTS tinkoff_investments.idx_shares_ticker;
DROP INDEX IF EXISTS tinkoff_investments.idx_shares_isin;
DROP INDEX IF EXISTS tinkoff_investments.idx_shares_country;
DROP INDEX IF EXISTS tinkoff_investments.idx_shares_sector;

-- Удаление таблицы акций
DROP TABLE IF EXISTS tinkoff_investments.shares CASCADE;

-- Удаление схемы
DROP SCHEMA IF EXISTS tinkoff_investments CASCADE;

-- Если есть какие-то зависимости, которые нужно также удалить
-- DROP VIEW IF EXISTS some_view_dependent_on_shares;
-- DROP FUNCTION IF EXISTS function_using_shares_data();