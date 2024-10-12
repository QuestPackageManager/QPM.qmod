use std::collections::HashSet;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(default)] // skip missing fields
pub struct ModJson {
    /// The Questpatcher version this mod.json was made for
    /// 1.1.0
    #[serde(rename(serialize = "_QPVersion", deserialize = "_QPVersion"))]
    pub schema_version: Version,
    /// Name of the mod
    pub name: String,
    /// ID of the mod
    pub id: String,
    /// Modloader. Possible values: QuestLoader/Scotland2
    pub modloader: Option<String>,
    /// Author of the mod
    pub author: String,
    /// Optional slot for if you ported a mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub porter: Option<String>,
    /// Mod version
    pub version: String,
    /// id of the package the mod is for, ex. com.beatgaems.beatsaber
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// Version of the package, ex. 1.1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    /// description for the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// optional cover image filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    /// whether or not this qmod is a library or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_library: Option<bool>,
    /// list of downloadable dependencies
    pub dependencies: Vec<ModDependency>,
    /// list of files that go in the package's early mods folder
    pub mod_files: Vec<String>,
    /// list of files that go in the package's late mods folder
    pub late_mod_files: Vec<String>,
    /// list of files that go in the package's libs folder
    pub library_files: Vec<String>,
    /// list of files that will be copied on the quest
    pub file_copies: Vec<FileCopy>,
    /// list of copy extensions registered for this specific mod
    pub copy_extensions: Vec<CopyExtension>,
}

impl Default for ModJson {
    fn default() -> Self {
        Self {
            schema_version: Version::new(1, 1, 0),
            name: Default::default(),
            id: Default::default(),
            author: Default::default(),
            porter: Default::default(),
            version: Default::default(),
            package_id: Default::default(),
            package_version: Default::default(),
            description: Default::default(),
            cover_image: Default::default(),
            is_library: Default::default(),
            dependencies: Default::default(),
            mod_files: Default::default(),
            library_files: Default::default(),
            file_copies: Default::default(),
            copy_extensions: Default::default(),
            modloader: Some("Scotland2".into()),
            late_mod_files: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModDependency {
    /// the version requirement for this dependency
    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    #[serde(rename = "version")]
    pub version_range: VersionReq,
    /// the id of this dependency
    pub id: String,
    /// the download link for this dependency, must satisfy id and version range!
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downloadIfMissing")]
    pub mod_link: Option<String>,
    /// whether the mod is optional or required. If omitted, assume Some(True)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "required")]
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileCopy {
    /// name of the file in the qmod
    pub name: String,
    /// place where to put it (full path)
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CopyExtension {
    /// the extension to register for
    pub extension: String,
    /// the destination folder these files should be going to
    pub destination: String,
}
