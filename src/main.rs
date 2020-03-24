use serde::{Deserialize};
use std::env;
use url::form_urlencoded::byte_serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let vejnavn = if let Some(cli_vejnavn) = env::args().nth(1) {
        cli_vejnavn
    }
    else {
        "Rentemestervej".to_string()
    };

    let husnr = if let Some(cli_husnr) = env::args().nth(2) {
        cli_husnr
    }
    else {
        "8".to_string()
    };

    
    //let txt = reqwest::get("https://dawa.aws.dk/adresser?vejnavn=Rentemestervej&husnr=8&etage=st&struktur=mini") // one
    let txt = reqwest::get("https://dawa.aws.dk/adresser?vejnavn=Rentemestervej&husnr=8&struktur=mini") // multiple
        .await?
        .json::<Vec<DawaAdresse>>()
        .await?;

    for x in txt.iter() {
        println!{"{:#?}", x};
    }
    
    let s_1 = "abc";
    let s_2 = "æø å";
    
    println!{"{}", &s_1};
    println!{"{}", &s_2};

    let url_1: String = byte_serialize(&s_1.as_bytes()).collect();
    let url_2: String = byte_serialize(&s_2.as_bytes()).collect();
    
    println!{"{:#?}", url_1};
    println!{"{:#?}", url_2};



    
    Ok(())
}

/// Dawa address struct
/// https://dawa.aws.dk/adresser?vejnavn=Rentemestervej&husnr=8&etage=st&struktur=mini
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
#[derive(Deserialize, Debug)]
pub struct DawaAdresse {
    id: String, // UUID
    status: u8,
    darstatus: u8,
    vejkode: String,
    vejnavn: String,
    adresseringsvejnavn: String,
    husnr: String,
    etage: Option<String>,
    //dør: Option<String>, // Non-ASCII
    supplerendebynavn: Option<String>,
    postnr: String,
    postnrnavn: String,
    stormodtagerpostnr: Option<i32>,
    stormodtagerpostnrnavn: Option<String>,
    kommunekode: String,
    adgangsadresseid: String, // UUID
    x: f64,
    y: f64,
    href: String,
    betegnelse: String,
}
