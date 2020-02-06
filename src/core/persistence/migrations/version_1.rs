use crate::prelude::*;
use rusqlite::Connection;

pub fn migrate(db: &Connection) -> Result<()> {
    info!("Applying migration…");

    // language=sql
    db.execute_batch(
        r#"
            PRAGMA synchronous = NORMAL;
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS sensors (
                pk INTEGER PRIMARY KEY NOT NULL,
                sensor_id TEXT UNIQUE NOT NULL,
                timestamp DATETIME NOT NULL
            );

            -- Stores all sensor readings.
            CREATE TABLE IF NOT EXISTS readings (
                sensor_fk INTEGER NOT NULL REFERENCES sensors(pk) ON UPDATE CASCADE ON DELETE CASCADE,
                timestamp DATETIME NOT NULL,
                value BLOB NOT NULL
            );
            -- Descending index on `timestamp` is needed to speed up the select last queries.
            CREATE UNIQUE INDEX IF NOT EXISTS readings_sensor_fk_timestamp ON readings (sensor_fk, timestamp DESC);

            PRAGMA user_version = 1;
        "#,
    )?;
    Ok(())
}
