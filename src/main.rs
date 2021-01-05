/// RocketApi - API Server
///
/// ## Example Usage
/// - Create the first super user
/// ```ignore
/// ./rocketapi createsuperuser -e user@gmail.com -f ./config.yaml
/// ```
/// - Start the server with a specific config file
/// ```ignore
/// ./rocketapi runserver -f ./config.yml
/// ```
/// - Start the Server with a default configuration
/// ```ignore
/// ./rocketapi runserver
/// ```
use clap::{crate_authors, crate_version, App, Arg, SubCommand};
use rocketapi::commands::{create_superuser, run_server};

fn main() {
    let matches = App::new("gatekeeper")
        .about(
            "A Gatekeeping service that controls api key access and manages access control lists",
        )
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("createsuperuser")
                .about("Create a super user.")
                .arg(
                    Arg::with_name("email")
                        .long("email")
                        .short("e")
                        .help("Email Address of super user to create")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("config_file")
                        .long("file")
                        .short("f")
                        .help("Config file path for db")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("runserver")
                .about("Run web server")
                .arg(
                    Arg::with_name("config_file")
                        .long("file")
                        .short("f")
                        .takes_value(true)
                        .help("Config file path for server"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("createsuperuser", Some(superuseruser_matches)) => {
            let email = superuseruser_matches.value_of("email").unwrap();
            let path = superuseruser_matches.value_of("config_file").unwrap();
            println!("Creating a superuser for {}", email);
            create_superuser(path, email, "command_line");
        }
        ("runserver", Some(runserver_matches)) => {
            let path = runserver_matches.value_of("config_file");
            println!("Running web server...");
            run_server(path);
        }
        ("", None) => println!("No subcommand passed!"),
        _ => unreachable!(),
    }
}
