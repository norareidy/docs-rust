use std::{ env, sync::Arc };

use bson::Document;
use mongodb::{
    Client,
    Collection,
    event::command::{ CommandEventHandler, CommandStartedEvent },
    options::ClientOptions,
};

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";

    // begin-command
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.command_event_handler = Some(EventHandler::callback(|ev| match ev {
        CommandEvent::Started(_) => {
            println!("{:?}", ev)
        }
        _ => (),
    }));
    
    let client = Client::with_options(client_options)?;

    // ... perform actions with the client to generate events

    // end-command

    Ok(())
}
