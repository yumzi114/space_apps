use module_bindings::*;


use spacetimedb_sdk::{credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey};

use crate::module_bindings;



const HOST: &str = env!("DB_HOST");

/// The database name we chose when we published our module.
const DB_NAME: &str = env!("DB_NAME");

/// Load credentials from a file and connect to the database.
pub fn connect_to_db() -> DbConnection {
    DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the process.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        // If the user has previously connected, we'll have saved a token in the `on_connect` callback.
        // In that case, we'll load it and pass it to `with_token`,
        // so we can re-authenticate as the same `Identity`.
        .with_token(creds_store().load().expect("Error loading credentials"))
        // Set the database name we chose when we called `spacetime publish`.
        .with_database_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(HOST)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect")
}

pub fn creds_store() -> credentials::File {
    credentials::File::new(env!("CHAT_KEY"))
}
/// Our `on_connect_error` callback: print the error, then exit the process.
pub fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
}

/// Our `on_disconnect` callback: print a note, then exit the process.
pub fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
    } else {
        println!("Disconnected.");
    }
}


/// Our `on_connect` callback: save our credentials to a file.
pub fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

/// Register all the callbacks our app will use to respond to database events.
pub fn register_callbacks(ctx: &DbConnection) {
    // When a new user joins, print a notification.
    ctx.db.user().on_insert(on_user_inserted);

    // When a user's status changes, print a notification.
    ctx.db.user().on_update(on_user_updated);

    // When a new message is received, print it.
    ctx.db.message().on_insert(on_message_inserted);
}
/// Our `User::on_insert` callback:
/// if the user is online, print a notification.
pub fn on_user_inserted(_ctx: &EventContext, user: &User) {
    if user.online {
        println!("User {} connected.", user_name_or_identity(user));
    }
}

pub fn user_name_or_identity(user: &User) -> String {
    user.name
        .clone()
        .unwrap_or_else(|| user.identity.to_hex().to_string())
}

/// Our `User::on_update` callback:
/// print a notification about name and status changes.
pub fn on_user_updated(_ctx: &EventContext, old: &User, new: &User) {
    if old.name != new.name {
        println!(
            "User {} renamed to {}.",
            user_name_or_identity(old),
            user_name_or_identity(new)
        );
    }
    if old.online && !new.online {
        println!("User {} disconnected.", user_name_or_identity(new));
    }
    if !old.online && new.online {
        println!("User {} connected.", user_name_or_identity(new));
    }
}
/// Our `Message::on_insert` callback: print new messages.
pub fn on_message_inserted(ctx: &EventContext, message: &Message) {
    if matches!(ctx.event, Event::Reducer(_) | Event::Transaction) {
        print_message(ctx, message)
    }
}

pub fn print_message(ctx: &impl RemoteDbContext, message: &Message) {
    let sender = ctx
        .db()
        .user()
        .identity()
        .find(&message.sender.clone())
        .map(|u| user_name_or_identity(&u))
        .unwrap_or_else(|| "unknown".to_string());
    println!("{}: {}", sender, message.text);
}

/// Register subscriptions for all rows of both tables.
pub fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .on_applied(on_sub_applied)
        .on_error(on_sub_error)
        .add_query(|q| q.from.user())
        .add_query(|q| q.from.message())
        .subscribe();
}

/// Our `on_subscription_applied` callback:
/// sort all past messages and print them in timestamp order.
pub fn on_sub_applied(ctx: &SubscriptionEventContext) {
    let mut messages = ctx.db.message().iter().collect::<Vec<_>>();
    messages.sort_by_key(|m| m.sent);
    for message in messages {
        print_message(ctx, &message);
    }
    println!("Fully connected and all subscriptions applied.");
    println!("Use /name to set your name, or type a message!");
}

/// Or `on_error` callback:
/// print the error, then exit the process.
pub fn on_sub_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Subscription failed: {}", err);
}
