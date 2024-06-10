use mongodb::{
    Client,
    event::EventHandler,
    event::cmap::CmapEvent,
    options::ClientOptions,
};

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";

    // begin-cmap
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.cmap_event_handler = Some(EventHandler::callback(|ev| match ev {
        CmapEvent::ConnectionCreated(_) => {
            println!("{:?}", ev)
        }
        _ => (),
    }));
    
    let client = Client::with_options(client_options)?;

    // ... perform actions with the client to generate events

    // end-cmap

    Ok(())
}
