//! # `tcrank`
//! `tcrank` is a tiny utility tool for THE IDOLM@STER MILLION LIVE!! THEATER DAYS and its special election event, THE@TER CHALLENGE!!.  
//! `tcrank` uses the [Princess](https://api.matsurihi.me/docs/).
//!
//! ## Install
//!
//! **`tcrank` is written in Rust. Thus you should install the latest Rust ecosystem in advance.**  
//! **refs. [rustup](https://rustup.rs/)**
//!
//! ### With `cargo install`
//!
//! ```
//! $ cargo install -f tcrank
//! ```
//!
//! ### Build from source code
//!
//! ```
//! $ git clone https://github.com/sadaie/tcrank
//! $ cd tcrank
//! $ cargo build --release
//! $ ls target/release/
//! build       deps        examples    incremental native      tcrank      tcrank.d
//! ```
//!
//! ## Usage
//!
//! ### Listing the idols and/or the roles.
//!
//! ```
//! # lists both of the idols and roles.
//! $ tcrank list
//!
//! # lists the idols.
//! $ tcrank list -i
//!
//! # lists the roles.
//! $ tcrank list -r
//! ```
//!
//! ### Showing the specified idol's rank(s).
//!
//! ```
//! # shows the idol's rank by ID.
//! $ tcrank show -i 21
//! Name        Role            Score  Rank
//! 徳川まつり  少女            80     9
//! 徳川まつり  魔法使い        10857  1
//! 徳川まつり  ファイナルデイ  36     7
//!
//! # shows the idol's rank by ID and role's ID.
//! $ tcrank show -i 21 -r 23
//! Name        Role            Score  Rank
//! 徳川まつり  魔法使い        10857  1
//!
//! # and you can use both of the idol's name and role's name.
//! $ tcrank show -i "徳川まつり" -r "魔法使い"
//! Name        Role            Score  Rank
//! 徳川まつり  魔法使い        10857  1
//! ```
//!
//! #### Additional options
//!
//! - `--json` option prints the result as `JSON` style string.
//! - `--json-pretty` option prints the result as pretty `JSON` style string.
//!
//! ## License
//!
//! MIT license.  

mod models;

use clap;
use env_logger;
use lazy_static;
use log::info;
use prettytable::{cell, format, row, Table};
use reqwest;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref IDOLS: HashMap<u8, (String, String)> = {
        include_str!("../data/idols.csv").lines().map(|l| {
            let values: Vec<&str> = l.split(',').collect();
            (values[0].parse::<u8>().expect("must be parsed"), (values[1].to_owned(), values[2].to_owned()))
        }).collect()
    };

    static ref ROLES: HashMap<u8, String> = {
        include_str!("../data/roles.csv").lines().map(|l| {
            let values: Vec<&str> = l.split(',').collect();
            (values[0].parse::<u8>().expect("must be parsed"), values[1].to_owned())
        }).collect()
    };
}

fn list_idols(should_insert_newline: bool) {
    let mut table = Table::from_csv_string(include_str!("../data/idols.csv")).expect("must exist");
    table.set_titles(row!["ID", "Full Name", "Personal Name"]);
    table.set_format(*format::consts::FORMAT_CLEAN);
    if should_insert_newline {
        println!("\nIdols:");
    } else {
        println!("Idols:");
    }
    table.printstd();
}

fn list_roles(should_insert_newline: bool) {
    let mut table = Table::from_csv_string(include_str!("../data/roles.csv")).expect("must exist");
    table.set_titles(row!["ID", "Full Name"]);
    table.set_format(*format::consts::FORMAT_CLEAN);
    if should_insert_newline {
        println!("\nRoles:");
    } else {
        println!("Roles:");
    }
    table.printstd();
}

fn get_data() -> Result<Vec<models::Ranking>, reqwest::Error> {
    reqwest::get("https://api.matsurihi.me/mltd/v1/election/current")
        .map(|mut r| r.json::<Vec<models::Ranking>>().expect("must be parsed."))
}

fn get_idol_rank(idol_ids: &[u8], role_ids: Option<&[u8]>) -> Option<Vec<models::Arrangement>> {
    let data: Vec<models::Arrangement> = if let Some(role_ids) = role_ids {
        get_data()
            .ok()?
            .iter()
            .filter(|r| role_ids.contains(&r.id()))
            .map(|r| {
                r.data()
                    .iter()
                    .filter(|p| idol_ids.contains(&p.idol().id()))
                    .map(|p| {
                        models::Arrangement::new(p.idol().name(), r.name(), p.score(), p.rank())
                    })
                    .collect::<Vec<models::Arrangement>>()
            })
            .flatten()
            .collect()
    } else {
        get_data()
            .ok()?
            .iter()
            .map(|r| {
                r.data()
                    .iter()
                    .filter(|p| idol_ids.contains(&p.idol().id()))
                    .map(|p| {
                        models::Arrangement::new(p.idol().name(), r.name(), p.score(), p.rank())
                    })
                    .collect::<Vec<models::Arrangement>>()
            })
            .flatten()
            .collect()
    };

    if data.is_empty() {
        None
    } else {
        Some(data)
    }
}

fn get_idol_id(id_or_name: &str) -> Option<u8> {
    match id_or_name.parse() {
        Ok(id) => {
            if IDOLS.contains_key(&id) {
                Some(id)
            } else {
                None
            }
        }
        Err(_) => IDOLS
            .iter()
            .filter(|(_, (f, p))| f == id_or_name || p == id_or_name)
            .map(|(k, _)| *k)
            .collect::<Vec<u8>>()
            .first()
            .cloned(),
    }
}

fn get_role_id(id_or_name: &str) -> Option<u8> {
    match id_or_name.parse() {
        Ok(id) => {
            if ROLES.contains_key(&id) {
                Some(id)
            } else {
                None
            }
        }
        Err(_) => ROLES
            .iter()
            .filter(|(_, n)| n.as_str() == id_or_name)
            .map(|(k, _)| *k)
            .collect::<Vec<u8>>()
            .first()
            .cloned(),
    }
}

fn main() {
    env_logger::init();

    let matches = clap::App::new(clap::crate_name!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            clap::SubCommand::with_name("list")
            .about("Lists the idols and the roles")
            .arg(
                clap::Arg::with_name("idols")
                .help("Lists the idols.")
                .short("i")
                .long("idols")
            )
            .arg(
                clap::Arg::with_name("roles")
                .help("Lists the roles.")
                .short("r")
                .long("roles")
            )
        )
        .subcommand(
            clap::SubCommand::with_name("show")
            .about("Shows specified idol rank")
            .arg(
                clap::Arg::with_name("idols")
                .help("Specifies the idol to show. See `tcrank list` to make sure the idol's ID or name.")
                .required(true)
                .short("i")
                .long("idols")
                .takes_value(true)
                .multiple(true)
                .value_name("IDOL_ID or IDOL_NAME")
            )
            .arg(
                clap::Arg::with_name("roles")
                .help("Specifies the role to show. See `tcrank list` to make sure the role's ID or name.")
                .short("r")
                .long("roles")
                .takes_value(true)
                .multiple(true)
                .value_name("ROLE_ID or ROLE_NAME")
            )
            .arg(
                clap::Arg::with_name("json")
                .help("Shows the output as JSON style string.")
                .short("j")
                .long("json")
            )
            .arg(
                clap::Arg::with_name("json_pretty")
                .help("Shows the output as pretty JSON style string.")
                .long("json-pretty")
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        match (matches.is_present("idols"), matches.is_present("roles")) {
            (true, true) | (false, false) => {
                list_idols(false);
                list_roles(true);
            }
            (true, false) => list_idols(false),
            (false, true) => list_roles(false),
        }
    } else if let Some(matches) = matches.subcommand_matches("show") {
        let idol_ids = matches
            .values_of("idols")
            .map(|id_or_names| {
                id_or_names
                    .map(|id_or_name| {
                        if let Some(id) = get_idol_id(id_or_name) {
                            id
                        } else {
                            eprintln!("The given idol ID or idol name is invalid.");
                            std::process::exit(1);
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .expect("must be set.");

        let role_ids = matches.values_of("roles").map(|values| {
            values
                .filter_map(|id_or_name| get_role_id(id_or_name))
                .collect::<Vec<u8>>()
        });

        if let Some(data) = get_idol_rank(&idol_ids, role_ids.as_ref().map(AsRef::as_ref)) {
            if matches.is_present("json_pretty") || matches.is_present("json") {
                let json = if matches.is_present("json_pretty") {
                    serde_json::to_string_pretty(&data)
                } else {
                    serde_json::to_string(&data)
                }
                .unwrap();
                println!("{}", json);
            } else {
                let mut table = Table::new();
                table.set_format(*format::consts::FORMAT_CLEAN);
                table.set_titles(row!["Name", "Role", "Score", "Rank"]);
                data.into_iter()
                    .map(|a| row![a.idol_name, a.role_name, a.score, a.rank])
                    .for_each(|r| {
                        table.add_row(r);
                    });
                table.printstd();
            }
        } else {
            info!("No data or network error");
        }
    }
}
