use crate::{config::Settings, server::start};

pub fn run_server(path: Option<&str>) {
    let settings = path
        .map(|p| Settings::from_file(p))
        .unwrap_or(Settings::default());
    start(settings);
}
