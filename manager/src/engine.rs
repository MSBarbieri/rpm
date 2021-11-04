use anyhow::Result;

use crate::Package;

#[derive(Clone, Debug)]
pub enum IoEvent {
    GetPackages,
    GetPackageInfo(Package),
    GetPackageHistory(Package),
    GetPackageDependencies(Package),
}

#[derive(Clone, Default, Debug)]
pub struct Engine {
    path: Vec<Package>,
    history_path: Vec<Package>,
}

impl Engine {
    pub fn new(path: String, history_path: String) -> Self {
        // path, history_path
        Engine {
            path: vec![],
            history_path: vec![],
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub async fn handle_package_event(&self, event: IoEvent) {
        match event {
            IoEvent::GetPackages => todo!(),
            IoEvent::GetPackageInfo(_) => todo!(),
            IoEvent::GetPackageHistory(_) => todo!(),
            IoEvent::GetPackageDependencies(_) => todo!(),
        };
    }

    pub async fn save() -> Result<()> {
        Ok(())
    }
}
