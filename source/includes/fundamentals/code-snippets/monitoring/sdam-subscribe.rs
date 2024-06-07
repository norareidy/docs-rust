use mongodb::{
    bson::{doc, Document},
    Client, Collection,
    event::EventHandler,
    event::sdam::{ SdamEventHandler, ServerOpeningEvent },
    options::ClientOptions,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    
    // begin-sdam
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.sdam_event_handler = Some(EventHandler::callback(|ev| println!("{:?}", ev)));
    
    let client = Client::with_options(client_options)?;

    // ... perform actions with the client to generate events

    // end-sdam
    
    Ok(())
}
