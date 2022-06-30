use metar::Metar;
use regex::Regex;

#[derive(Debug)]
enum MTError {
    RequestError(reqwest::Error),
    MetarParseError(metar::MetarError),
}

impl From<reqwest::Error> for MTError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}

impl From<metar::MetarError> for MTError {
    fn from(e: metar::MetarError) -> Self {
        Self::MetarParseError(e)
    }
}

fn get_metar(station: &str) -> Result<String, MTError> {
    let body = reqwest::blocking::get(&format!("https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&hoursBeforeNow=3&mostRecent=true&stationString={}", station))?
        .text()?;

    let re = Regex::new(r"<raw_text>([^<]+)</raw_text>").unwrap();
    let caps = re.captures(&body).unwrap();
    let metar_text = &caps[1];
    Ok(metar_text.to_string())
}

fn main() {
    let test_stations = [
        "KLAX", "EGSS", "LTBJ", "EDDK", "EGMC", "EGGD", "ESSA", "EGPC", "RJAA", "UUDD", "FACT",
        "ZGSZ",
    ];

    let mut num_errors = 0;

    for station in &test_stations {
        let res = get_metar(station);
        if let Ok(metar) = res {
            println!("Testing METAR: {}", metar.clone());
            let r = Metar::parse(&metar);
            if let Err(e) = r {
                eprintln!("Error:");
                eprintln!("{}", e);
                num_errors += 1;
            } else if let Ok(res) = r {
                println!("{:?}", res);
            }
        } else {
            let res = res.unwrap_err();
            eprintln!("Error fetching data for testing {}", station);
            eprintln!("{:#?}", res);
        }
        println!("\n-----\n");
    }

    if num_errors > 0 {
        std::process::exit(1);
    }
}
