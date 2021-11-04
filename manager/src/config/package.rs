use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageManager {
    pub package_list_file_path: String,
    pub history_path: String,
}
impl Default for PackageManager {
    fn default() -> Self {
        PackageManager {
            package_list_file_path: String::from("/etc/rpm/packages.yaml"),
            history_path: String::from("/etc/rpm/packages_history.yaml"),
        }
    }
}
