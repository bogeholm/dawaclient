use serde::Deserialize;
use std::env;
use url::form_urlencoded::byte_serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let vejnavn = if let Some(cli_vejnavn) = env::args().nth(1) {
        cli_vejnavn
    } else {
        "Rentemestervej".to_string()
    };

    let husnr = if let Some(cli_husnr) = env::args().nth(2) {
        cli_husnr
    } else {
        "8".to_string()
    };

    // Construct request
    let request_url = format! {"https://dawa.aws.dk/adresser?vejnavn={}&husnr={}&struktur=mini",
    url_enc(&vejnavn), husnr};

    // Request to DAWA
    let addresses = reqwest::get(&request_url)
        .await?
        .json::<Vec<DawaAdresse>>()
        .await?;

    // adress.betegnelse is human friendly
    for address in addresses.iter() {
        println! {"{}", address.betegnelse};
    }

    Ok(())
}

/// URL encode an `&str`
fn url_enc(string: &str) -> String {
    byte_serialize(string.as_bytes()).collect()
}

/// Dawa address struct
/// https://dawa.aws.dk/adresser?vejnavn=Rentemestervej&husnr=8&etage=st&struktur=mini
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
pub struct DawaAdresse {
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
