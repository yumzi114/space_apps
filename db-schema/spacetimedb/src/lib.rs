use std::time::Duration;

use spacetimedb::{Identity, ReducerContext, ScheduleAt, Table, Timestamp, reducer, table};


#[table(accessor = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[table(accessor = message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    id: u64,
    sender: Identity,
    sent: Timestamp,
    text: String,
}
#[table(accessor = cleanup_schedule, scheduled(clear_old_messages))]
pub struct CleanupSchedule {
    #[primary_key]
    #[auto_inc]
    id: u64,
    scheduled_at: ScheduleAt,
}
#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    if ctx.db.cleanup_schedule().iter().next().is_none() {
        ctx.db.cleanup_schedule().insert(CleanupSchedule {
            id: 0,
            scheduled_at: ScheduleAt::Interval(Duration::from_secs(60).into()),
        });
    }
}
#[reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User { name: Some(name), ..user });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}


#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    ctx.db.message().insert(Message {
        id: 0,
        sender: ctx.sender(),
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User { online: true, ..user });
    } else {
        ctx.db.user().insert(User {
            name: None,
            identity: ctx.sender(),
            online: true,
        });
    }
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User { online: false, ..user });
    } else {
        log::warn!("Disconnect event for unknown user with identity {:?}", ctx.sender());
    }
}


#[reducer]
pub fn clear_old_messages(ctx: &ReducerContext, _job: CleanupSchedule) {
    let cutoff = ctx.timestamp - Duration::from_secs(60 * 60); // 1시간

    let old_ids: Vec<u64> = ctx.db.message()
        .iter()
        .filter(|m| m.sent < cutoff)
        .map(|m| m.id)
        .collect();

    for id in old_ids {
        ctx.db.message().id().delete(id);
    }

    log::info!("old messages cleared");
}