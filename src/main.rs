use http::status::StatusCode;
use serde::Deserialize;
use std::{env, error::Error};
use uuid::Uuid;

const USAGE: &str = "Usage: `dawaclient <street name> <house number>`";

/// Parses CLI arguments, sends a request to [DAWA](https://dawa.aws.dk/) and prints the results. The
/// expected CLI args are `street name` and `house number`:
///```bash
/// dawaclient <street name> <house number>
/// ```
/// Any matching adresses are printed to `stdout`, any errors are returned as well.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let cli_args = parse_cli()?;

    // Construct request
    let request_url = format!(
        "https://dawa.aws.dk/adresser?vejnavn={}&husnr={}&struktur=mini",
        cli_args.street_name, cli_args.house_number
    );

    // Request to DAWA
    println!("Sending request to {}", &request_url);
    let response = reqwest::get(&request_url).await?;
    println!("HTTP status: {}", response.status());

    // Parse response
    let addresses = parse_response(response).await?;

    // `betegnelse` is human friendly
    println!("Found {} address(es)\n", addresses.len());
    for address in addresses.iter() {
        println!("{}", address.betegnelse);
    }

    Ok(())
}

/// Consume the [`reqwest::Response`](https://docs.rs/reqwest/0.10.4/reqwest/struct.Response.html)
/// and parse as a Vec of DawaAddresses (which may be empty) on 200 OK, or else return any errors.
/// Note that [DAWA's errors are structured](https://dawa.aws.dk/dok/api/generelt#fejlhaandtering),
/// so any error could potentially be parsed and handled case by case.
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

/// Parse CLI arguments. `street name` and `house number` are expected, in that order.
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

/// Captures the CLI arguments `street name` and `house number`.
#[derive(Debug)]
#[non_exhaustive]
struct CliArgs {
    street_name: String,
    house_number: String,
}

/// Minimal generic custom error type that wraps [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html)
#[derive(Debug)]
struct DawaError(String);

impl std::fmt::Display for DawaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for DawaError {}

/// Dawa ['mini'](https://dawa.aws.dk/dok/api/generelt#fladognestede) address struct.
/// Example: [https://dawa.aws.dk/adresser?vejnavn=Rentemestervej&husnr=8&etage=st&struktur=mini](https://dawa.aws.dk/adresser?vejnavn=Rentemestervej&husnr=8&etage=st&struktur=mini)
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
/// Types are based on cursory inspection of a handful of responses.
#[derive(Deserialize, Debug)]
#[non_exhaustive]
struct DawaAddress {
    id: Uuid,
    status: u16,
    darstatus: u16,
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
    stormodtagerpostnr: Option<String>,
    stormodtagerpostnrnavn: Option<String>,
    kommunekode: String,
    adgangsadresseid: Uuid,
    x: f64,
    y: f64,
    href: String,
    betegnelse: String,
}
