/// Lists the id's and dimensions of all image assets in a given Sanity.io project and dataset.
///
/// Usage: assets [project-id] [dataset]
///
use sanity::Client;
use serde::Deserialize;
use std::fmt;
//
// In this example, we use serde_json to deserialize the response from the Sanity.io query, so we have to
// define the structure we expect to receive.
//
#[derive(Deserialize, Debug)]
struct Dimensions {
    width: u32,
    height: u32,
}
#[derive(Deserialize, Debug)]
struct Asset {
    id: String,
    dimensions: Dimensions,
}
impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}x{})", self.width, self.height)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("{} [project-id] [dataset]", &args[0]);
        return Ok(());
    }
    let project = &args[1];
    let dataset = &args[2];
    //
    // Create a new client.
    //
    // Uses a builder-like pattern; project-id, dataset, and api-version are required. Append
    // `.use_cdn(false)` bypass the API CDN. Authorization token can be specified using `.with_token("your-token")`
    //
    let client = Client::new(project, dataset, "2022-01-12");
    //
    // Build & execute the query.
    //
    let response = client
        .query("*[ _type == 'sanity.imageAsset' ]{ 'id': _id, 'dimensions': metadata.dimensions }")
        .json::<Asset>()
        .await?;

    for Asset { id, dimensions } in response.result {
        println!("{}:\t{}", id, dimensions);
    }

    Ok(())
}
