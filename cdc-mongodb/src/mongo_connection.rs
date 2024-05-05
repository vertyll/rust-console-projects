use mongodb::{Client, options::{ClientOptions, ReadConcern}};

pub async fn connect_to_mongo(mongo_url: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(mongo_url).await?;
    client_options.read_concern = Some(ReadConcern::majority());
    Client::with_options(client_options)
}
