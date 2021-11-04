#[derive(Debug, Clone, Default)]
pub struct Version {
    hash: String,
    tag: Option<String>,
    commit: String,
}

impl Version {
    pub fn new(hash: String, tag: Option<String>, commit: String) -> Self {
        Version { hash, tag, commit }
    }

    pub async fn is_latest_version(&self, versions: Vec<Version>) -> bool {
        todo!("implement is latest version")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Package {
    title: String,
    uri: String,
    installed_version: Version,
}

impl Package {
    pub fn new(title: String, uri: String, installed_version: Version) -> Self {
        Package {
            title,
            uri,
            installed_version,
        }
    }

    pub async fn get_list_versions(&self) -> Vec<Version> {
        todo!("implement get list versions");
    }
}
