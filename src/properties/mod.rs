use std::sync::Arc;

use crate::domain;

mod configuration_loader;

shaku::module! {
    pub PropertiesModule: domain::PropertiesModule {
        components = [
            configuration_loader::ConfigurationLoader,
        ],
        providers = [],
    }
}
pub fn module() -> Arc<dyn domain::PropertiesModule> {
    Arc::new(PropertiesModule::builder().build())
}