WITH target_instruments AS (
    SELECT uid, figi
    FROM instrument_services.share
    WHERE figi IN ('BBG004730N88', 'BBG004731354', 'BBG004731032')
),
interval_id AS (
    SELECT id 
    FROM instrument_services.subscription_intervals 
    WHERE name = 'SUBSCRIPTION_INTERVAL_ONE_MINUTE'
)
INSERT INTO instrument_services.watched_instruments 
    (uid, figi, subscription_interval_id, store_history_days, notes)
SELECT 
    uid,
    figi,
    (SELECT id FROM interval_id),
    30,
    'Added during initial migration'
FROM target_instruments;