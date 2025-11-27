#![deny(clippy::pedantic)]
#![deny(unsafe_code)]

use anyhow::anyhow;
use metar::Metar;
use regex::Regex;

fn get_metar(station: &str) -> anyhow::Result<String> {
    let body = reqwest::blocking::get(format!(
        "https://aviationweather.gov/api/data/metar?ids={station}"
    ))?
    .text()?;

    let re = Regex::new(r"^METAR (.+)$")?;
    let caps = re.captures(&body).ok_or(anyhow!("No METAR present"))?;
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
            if let Err(es) = r {
                eprintln!("Errors:");
                for e in es {
                    eprintln!("{e}");
                }
                num_errors += 1;
            } else if let Ok(res) = r {
                println!("{res:?}");
            }
        } else {
            let res = res.unwrap_err();
            eprintln!("Error fetching data for testing {station}");
            eprintln!("{res:#?}");
        }
        println!("\n-----\n");
    }

    if num_errors > 0 {
        std::process::exit(1);
    }
}
