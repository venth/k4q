use std::path::PathBuf;

use home::home_dir;
use shaku;

use crate::domain::ports;

#[derive(shaku::Component)]
#[shaku(interface = ports::PropertiesLocationProvider)]
pub struct CliPropertiesLocationProvider {}

impl ports::PropertiesLocationProvider for CliPropertiesLocationProvider {
    fn provide(&self, _args: &Vec<&str>) -> Option<PathBuf> {
        home_dir()
            .map(|p| p.join(".k4fq/config.yaml"))
    }
}