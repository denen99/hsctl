extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};

mod api;
mod device;
mod http;
mod output;

fn main() {

    let matches = App::new("hsctl")
        .about("A CLI tool for interacting with HomeSeer")
        .version("0.1")
        .author("Adam Denenberg")
        .arg(
            Arg::with_name("output")
                .help("Specify output format, defaults to table")
                .short("o")
                .long("output")
                .required(false)
                .default_value("table")
                .possible_values(&["table", "json"]),
        )
        .arg(
            Arg::with_name("hostname")
                .help("Specify hostname to connect to")
                .short("h")
                .long("hostname")
                .required(false)
                .default_value("https://connected11.homeseer.com"),
        )

        .subcommand(
            SubCommand::with_name("status")
                .arg(
                    Arg::with_name("ref")
                        .help("Filter by reference id, defaults to ALL")
                        .long("ref")
                        .short("r")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("location1")
                        .help("Filter by location1, defaults to ALL")
                        .long("loc1")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("location2")
                        .help("Filter by location2, defaults to ALL")
                        .long("loc2")
                        .takes_value(true)
                        .required(false),
                )
                .about("Get Status of devices"),
        )
        .subcommand(
            SubCommand::with_name("control")
                .arg(
                    Arg::with_name("ref")
                        .help("Reference ID to control")
                        .long("ref")
                        .short("r")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("label")
                        .help("Control device by label (Note: cannot be used with option value)")
                        .long("label")
                        .conflicts_with("value")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("value")
                        .help("Control device by value (Note: cannot be used with option label)")
                        .long("value")
                        .conflicts_with("label")
                        .required(false)
                        .takes_value(true),
                )
                .about("Control devices by label or by value"),
        )
        .subcommand(
            SubCommand::with_name("login")
                .about("Login to HomeSeer and save token")
                .arg(
                    Arg::with_name("username")
                        .help("Username you login to HomeSeer with")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("password")
                        .help("Password you login to HomeSeer with")
                        .index(2)
                        .required(true),
                ),
        )
        .get_matches();

    let help = matches.usage();

    if matches.subcommand_name().is_none() {
        println!("{}", help);
        return;
    }

    // Grab the output format, defaults to table
    let output_type = matches.value_of("output").unwrap();
    let hostname = matches.value_of("hostname").unwrap();

    let mut api = http::APIRequest::new(String::from(hostname), output_type);

    // Process login command
    if let Some(sub) = matches.subcommand_matches("login") {
        let username = sub.value_of("username").unwrap();
        let password = sub.value_of("password").unwrap();
        api.login(username, password);
        return;
    } else {
        api.login_with_saved_token();
    }

    match matches.subcommand_name() {
        Some("login") => {
            let username = matches
                .subcommand_matches("login")
                .unwrap()
                .value_of("username")
                .unwrap();
            let password = matches
                .subcommand_matches("login")
                .unwrap()
                .value_of("password")
                .unwrap();
            api.login(username, password);
        }

        Some("control") => {
            let ref_id: u32 = match matches
                .subcommand_matches("control")
                .unwrap()
                .value_of("ref")
            {
                Some(x) => x.parse().unwrap(),
                _ => panic!("Invalid ref_id of 0"),
            };

            let v: f32 = match matches
                .subcommand_matches("control")
                .unwrap()
                .value_of("value")
            {
                Some(x) => x.parse::<f32>().unwrap(),
                _ => -1 as f32,
            };

            let l: String = match matches
                .subcommand_matches("control")
                .unwrap()
                .value_of("label")
            {
                Some(x) => x.to_string(),
                _ => String::from(""),
            };

            api.set_status(ref_id, l, v);
        }

        Some("status") => {
            let ref_id: u32 = match matches
                .subcommand_matches("status")
                .unwrap()
                .value_of("ref")
            {
                Some(x) => x.parse().unwrap(),
                _ => 0,
            };

            let loc1 = match matches
                .subcommand_matches("status")
                .unwrap()
                .value_of("location1")
            {
                Some(x) => x,
                _ => "all",
            };

            let loc2 = match matches
                .subcommand_matches("status")
                .unwrap()
                .value_of("location2")
            {
                Some(x) => x,
                _ => "all",
            };

            api.get_status(ref_id, loc1, loc2);
        }
        _ => println!("{}", help),
    }

}
