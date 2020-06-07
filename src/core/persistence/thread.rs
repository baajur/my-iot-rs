use crate::prelude::*;

/// Spawn the persistence thread.
pub fn spawn(db: Connection, bus: &mut Bus) -> Result<()> {
    info!("Spawning readings persistence…");
    let rx = bus.add_rx();

    thread::Builder::new()
        .name("system::persistence".into())
        .spawn(move || {
            for message in &rx {
                if let Err(error) = process_message(&message, &db) {
                    error!("{}: {:?}", error, &message);
                }
            }
            unreachable!();
        })?;

    Ok(())
}

/// Process a message.
fn process_message(message: &Message, db: &Connection) -> Result<()> {
    info!(
        "[{:?}] {} = {:?}",
        &message.type_, &message.sensor.id, &message.reading.value
    );
    debug!("{:?}", &message);
    // TODO: handle `ReadSnapshot` properly.
    if message.type_ == MessageType::ReadLogged || message.type_ == MessageType::ReadSnapshot {
        message.upsert_into(&db)?;
    }
    Ok(())
}
