/// RocketApi - API Server
///
/// ## Example Usage
/// - Create the first super user
/// ```ignore
/// ./rocketapi --createsuperuser
/// ```
/// - Start the server with a specific config file
/// ```ignore
/// ./rocketapi --config=config.sample.yml
/// ```
/// - Start the Server with a default configuration
/// ```ignore
/// ./rocketapi
/// ```
use clap::{crate_authors, crate_version, Clap};
use rocketapi::{server::config::Settings, server::start};

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct CliOpts {
    #[clap(
        short,
        long,
        conflicts_with = "createsuperuser",
        about = "loads the api server configurations"
    )]
    config: Option<String>,

    #[clap(
        short = 'u',
        long,
        conflicts_with = "config",
        about = "creates a superuser api account"
    )]
    createsuperuser: bool,
}

fn main() {
    //! Start of rocketapi
    // parse the cli options
    let cli_opts = CliOpts::parse();
    let cfg_file = &cli_opts.config.unwrap_or("".to_string());
    if cli_opts.createsuperuser {
        // create super user command invoked
        println!("create super user invoked");
    } else {
        // start rocket server with appropriate
        // settings from config yaml file
        let settings = if cfg_file.is_empty() {
            Settings::default()
        } else {
            Settings::from_file(&cfg_file)
        };
        start(settings);
    }
}
