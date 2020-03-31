use http::status::StatusCode;
use serde::Deserialize;
use std::{env, error::Error};

const USAGE: &str = "Usage: `dawaclient <street name> <house number>`";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let cli_args = parse_cli()?;

    // Construct request
    let request_url = format! {"https://dawa.aws.dk/adresser?vejnavn={}&husnr={}&struktur=mini",
    cli_args.street_name, cli_args.house_number};
    println! {"Sending request to {}", &request_url};

    // Request to DAWA
    let response = reqwest::get(&request_url).await?;

    // Parse response
    let addresses = parse_response(response).await?;

    // `betegnelse` is human friendly
    println! {"Found {} address(es)\n", addresses.len()};
    for address in addresses.iter() {
        println! {"{}", address.betegnelse};
    }

    Ok(())
}

/// Consume the web response and parse as a Vec of DawaAddresses (which may be empty) on 200 OK,
/// or else return the error from DAWA.
async fn parse_response(response: reqwest::Response) -> Result<Vec<DawaAddress>, Box<dyn Error>> {
    let status = response.status();
    let body = response.text().await?;

    match status {
        StatusCode::OK => {
            let addresses: Vec<DawaAddress> = serde_json::from_str(&body)?;
            Ok(addresses)
        }
        _ => Err(Box::new(DawaError(body))),
    }
}

/// Expected CLI arguments are street name and house number
fn parse_cli() -> Result<CliArgs, DawaError> {
    let street_name = if let Some(cli_street_name) = env::args().nth(1) {
        cli_street_name
    } else {
        return Err(DawaError(USAGE.into()));
    };

    let house_number = if let Some(cli_house_number) = env::args().nth(2) {
        cli_house_number
    } else {
        return Err(DawaError(USAGE.into()));
    };

    Ok(CliArgs {
        street_name,
        house_number,
    })
}

#[derive(Debug)]
#[non_exhaustive]
pub struct CliArgs {
    street_name: String,
    house_number: String,
}

/// Generic custom error type.
#[derive(Debug)]
struct DawaError(String);

impl std::fmt::Display for DawaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for DawaError {}

/// Dawa address struct. Example: [https://dawa.aws.dk/adresser?street_name=Rentemestervej&house_number=8&etage=st&struktur=mini](https://dawa.aws.dk/adresser?street_name=Rentemestervej&house_number=8&etage=st&struktur=mini)
/// ```javascript
/// [
///     {
///     "id": "0a3f50a0-4660-32b8-e044-0003ba298018",
///     "status": 1,
///     "darstatus": 3,
///     "vejkode": "5804",
///     "vejnavn": "Rentemestervej",
///     "adresseringsvejnavn": "Rentemestervej",
///     "husnr": "8",
///     "etage": "st",
///     "dør": null,
///     "supplerendebynavn": null,
///     "postnr": "2400",
///     "postnrnavn": "København NV",
///     "stormodtagerpostnr": null,
///     "stormodtagerpostnrnavn": null,
///     "kommunekode": "0101",
///     "adgangsadresseid": "0a3f507a-e179-32b8-e044-0003ba298018",
///     "x": 12.53547185,
///     "y": 55.70481955,
///     "href": "https://dawa.aws.dk/adresser/0a3f50a0-4660-32b8-e044-0003ba298018",
///     "betegnelse": "Rentemestervej 8, st., 2400 København NV"
///   }
/// ]
/// ```
#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct DawaAddress {
    id: String, // GUID
    status: u8,
    darstatus: u8,
    vejkode: String,
    vejnavn: String,
    adresseringsvejnavn: String,
    husnr: String,
    etage: Option<String>,
    #[serde(rename = "dør")] // Non-ASCII
    doer: Option<String>,
    supplerendebynavn: Option<String>,
    postnr: String,
    postnrnavn: String,
    stormodtagerpostnr: Option<i32>,
    stormodtagerpostnrnavn: Option<String>,
    kommunekode: String,
    adgangsadresseid: String, // GUID
    x: f64,
    y: f64,
    href: String,
    betegnelse: String,
}
