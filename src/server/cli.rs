use clap::{crate_authors, crate_version, Clap};

use std::net::IpAddr;

#[derive(Clap, Debug, Serialize, Deserialize)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct CliOpts {
    #[clap(short, long)]
    pub config: Option<String>,

    #[clap(short, long, default_value = "0.0.0.0")]
    pub host: IpAddr,

    #[clap(short, long, default_value = "8000")]
    pub port: u16,

    #[clap(long)]
    pub enable_tls: bool,
}
