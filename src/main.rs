use reqwest;
use regex::Regex;
use metar::Metar;

#[derive(Debug)]
enum MTError<'a> {
    RequestError(reqwest::Error),
    MetarParseError(metar::MetarError<'a>),
}

impl From<reqwest::Error> for MTError<'_> {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}

impl<'a> From<metar::MetarError<'a>> for MTError<'a> {
    fn from(e: metar::MetarError<'a>) -> Self {
        Self::MetarParseError(e)
    }
}

fn get_metar(station: &str) -> Result<String, MTError<'_>> {
    let body = reqwest::blocking::get(&format!("https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&hoursBeforeNow=3&mostRecent=true&stationString={}", station))?
        .text()?;

    let re = Regex::new(r"<raw_text>([^<]+)</raw_text>").unwrap();
    let caps = re.captures(&body).unwrap();
    let metar_text = &caps[1];
    Ok(metar_text.to_string())
}

fn main() {
    let test_stations = ["KLAX", "EGSS", "LTBJ", "EDDK", "EGMC", "EGGD", "ESSA", "EGPC", "RJAA", "UUDD", "FACT", "ZGSZ"];

    for station in &test_stations {
        let res = get_metar(station);
        if let Ok(metar) = res {
            println!("Testing METAR: {}", metar.clone());
            let r = Metar::parse(&metar);
            if let Err(e) = r {
                eprintln!("Error parsing:");
                eprintln!("{}", e);
                eprintln!("-----");
            }
        } else {
            let res = res.unwrap_err();
            eprintln!("Error fetching data for testing {}", station);
            eprintln!("{:#?}", res);
            eprintln!("-----");
        }
    }
}
