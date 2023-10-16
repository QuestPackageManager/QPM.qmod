use std::collections::HashSet;

use qpm_package::{
    extensions::package_metadata::PackageMetadataExtensions,
    models::dependency::{Dependency, SharedDependency, SharedPackageConfig},
};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(default)] // skip missing fields
pub struct ModJson {
    /// The Questpatcher version this mod.json was made for
    #[serde(rename(serialize = "_QPVersion", deserialize = "_QPVersion"))]
    pub schema_version: Version,
    /// Name of the mod
    pub name: String,
    /// ID of the mod
    pub id: String,
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
    /// list of files that go in the package's mod folder
    pub mod_files: Vec<String>,
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
            schema_version: Version::new(1, 0, 0),
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

impl From<SharedPackageConfig> for ModJson {
    fn from(shared_package: SharedPackageConfig) -> Self {
        let local_deps = &shared_package.config.dependencies;

        // Only bundle mods that are not specifically excluded in qpm.json or if they're not header-only
        let restored_deps: Vec<_> = shared_package
            .restored_dependencies
            .iter()
            .filter(|dep| {
                let local_dep_opt = local_deps
                    .iter()
                    .find(|local_dep| local_dep.id == dep.dependency.id);

                if let Some(local_dep) = local_dep_opt {
                    // if force included/excluded, return early
                    if let Some(force_included) = local_dep.additional_data.include_qmod {
                        return force_included;
                    }
                }

                // or if header only is false
                dep.dependency.additional_data.mod_link.is_some()
                    || !dep.dependency.additional_data.headers_only.unwrap_or(false)
            })
            .collect();

        // List of dependencies we are directly referencing in qpm.json
        let direct_dependencies: HashSet<String> = shared_package
            .config
            .dependencies
            .iter()
            .map(|f| f.id.clone())
            .collect();

        // downloadable mods links n stuff
        // mods that are header-only but provide qmods can be added as deps
        // Must be directly referenced in qpm.json
        let mods: Vec<ModDependency> = local_deps
            .iter()
            // Removes any dependency without a qmod link
            .filter_map(|dep| {
                let shared_dep = restored_deps.iter().find(|d| d.dependency.id == dep.id);
                if shared_dep.is_none() {
                    return None;
                }
                if shared_dep
                    .unwrap()
                    .dependency
                    .additional_data
                    .mod_link
                    .is_some()
                {
                    return Some((shared_dep.unwrap(), dep));
                }

                None
            })
            .map(|(shared_dep, dep)| ModDependency {
                version_range: dep.version_range.clone(),
                id: dep.id.clone(),
                mod_link: shared_dep.dependency.additional_data.mod_link.clone(),
            })
            .collect();

        // The rest of the mods to handle are not qmods, they are .so or .a mods
        // actual direct lib deps
        let libs: Vec<String> = shared_package
            .restored_dependencies
            .iter()
            // We could just query the bmbf core mods list on GH?
            // https://github.com/BMBF/resources/blob/master/com.beatgames.beatsaber/core-mods.json
            // but really the only lib that never is copied over is the modloader, the rest is either a downloaded qmod or just a copied lib
            // even core mods should technically be added via download
            .filter(|lib|
                // Must be directly referenced in qpm.json
                direct_dependencies.contains(&lib.dependency.id) &&

                // keep if header only is false, or if not defined
                !lib.dependency.additional_data.headers_only.unwrap_or(false) &&

                // Modloader should never be included
                lib.dependency.id != "modloader" && 
                
                // don't include static deps
                !lib.dependency.additional_data.static_linking.unwrap_or(false) &&

                // it's marked to be included, defaults to including ( same as dependencies with qmods )
                lib.dependency.additional_data.include_qmod.unwrap_or(true) &&

                // Only keep libs that aren't downloadable
                !mods.iter().any(|dep| lib.dependency.id == dep.id))
            .map(|dep| dep.get_so_name().to_str().unwrap().to_string())
            .collect();

        Self {
            schema_version: Version::new(1, 0, 0),
            name: shared_package.config.info.name.clone(),
            id: shared_package.config.info.id.clone(),
            author: Default::default(),
            porter: None,
            version: shared_package.config.info.version.to_string(),
            package_id: None,
            package_version: None,
            description: None,
            cover_image: None,
            is_library: None,
            dependencies: mods,
            mod_files: vec![shared_package
                .config
                .info
                .get_so_name()
                .to_str()
                .unwrap()
                .to_string()],
            library_files: libs,
            file_copies: Default::default(),
            copy_extensions: Default::default(),
        }
    }
}

impl From<Dependency> for ModDependency {
    fn from(dep: Dependency) -> Self {
        Self {
            id: dep.id,
            version_range: dep.version_range,
            mod_link: dep.additional_data.mod_link,
        }
    }
}

impl From<SharedDependency> for ModDependency {
    fn from(dep: SharedDependency) -> Self {
        dep.dependency.into()
    }
}
