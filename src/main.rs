#[tokio::main(flavor = "current_thread")]
async fn main() {
    let uri = kuksa::Uri::from_static("http://127.0.0.1:55555");
    let mut client = kuksa::Client::new(uri);
    match client
        .get_current_values(vec!["Vehicle.Speed".into()])
        .await
    {
        Ok(entries) => match entries.first() {
            Some(kuksa::DataEntry { value, .. }) => match value {
                Some(kuksa::proto::v1::Datapoint { value, .. }) => match value {
                    Some(kuksa::proto::v1::datapoint::Value::Float(value)) => {
                        println!("Vehicle.Speed: {:?}", value)
                    }
                    Some(_) => println!("Unexpected data type"),
                    None => println!("Vehicle.Speed: Not available"),
                },
                None => {
                    println!("No value included in entry");
                }
            },
            None => println!("No matching entry"),
        },
        Err(err) => println!("Error: {:?}", err),
    };
}
