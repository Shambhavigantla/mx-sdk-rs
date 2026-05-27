use crate::{cli::TemplateArgs, version::FrameworkVersion, version_history::LAST_TEMPLATE_VERSION};

use super::{
    ContractCreatorTarget, RepoSource, RepoVersion, TemplateAdjuster,
    template_source::{TemplateSource, template_sources},
};
use multiversx_sc_meta_lib::cargo_toml::CargoTomlContents;
use pathdiff::diff_paths;
use std::path::PathBuf;
use toml::{Value as TomlValue, value::Table as TomlTable};

/// Creates a new contract on disk, from a template, given a name.
pub async fn create_contract(args: &TemplateArgs) {
    let version = get_repo_version(&args.tag);
    let version_tag: FrameworkVersion = version.get_tag();
    let repo_temp_download = RepoSource::download_from_github(version, std::env::temp_dir()).await;
    let target = target_from_args(args);

    let creator = ContractCreator::new(
        &repo_temp_download,
        args.template.clone(),
        target,
        false,
        args.author.clone(),
    );

    creator.create_contract(version_tag);
}

fn target_from_args(args: &TemplateArgs) -> ContractCreatorTarget {
    let target_path = args.path.clone().unwrap_or_default();
    let new_name = args.name.as_deref().unwrap_or(&args.template);
    ContractCreatorTarget::new(target_path, new_name)
}

pub(crate) fn get_repo_version(args_tag: &Option<String>) -> RepoVersion {
    if let Some(tag) = args_tag {
        RepoVersion::Tag(tag.clone())
    } else {
        RepoVersion::Tag(LAST_TEMPLATE_VERSION.to_string())
    }
}

/// Coordinates the creation of a new contract from a template.
pub struct ContractCreator<'a> {
    pub repo_source: &'a RepoSource,
    pub template_source: TemplateSource<'a>,
    pub target: ContractCreatorTarget,
    pub adjuster: TemplateAdjuster,
}

impl<'a> ContractCreator<'a> {
    pub fn new(
        repo_source: &'a RepoSource,
        template_name: String,
        target: ContractCreatorTarget,
        keep_paths: bool,
        new_author: Option<String>,
    ) -> Self {
        let template_sources = template_sources(repo_source);
        let template_source = template_sources
            .into_iter()
            .find(|source| source.metadata.name == template_name)
            .unwrap_or_else(|| panic!("Unknown template {template_name}"));

        let metadata = template_source.metadata.clone();
        ContractCreator {
            repo_source,
            template_source,
            target: target.clone(),
            adjuster: TemplateAdjuster {
                metadata,
                target,
                keep_paths,
                new_author,
            },
        }
    }

    pub fn create_contract(&self, args_tag: FrameworkVersion) {
        self.copy_template(args_tag.clone());
        self.update_dependencies(args_tag);
        self.rename_template();
            self.inject_repo_patch_if_local();
    }

        fn inject_repo_patch_if_local(&self) {
            use super::RepoSource;

            match self.repo_source {
                RepoSource::LocalPath(repo_root) => {
                    let cargo_toml_path = self.target.contract_dir().join("Cargo.toml");
                    let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

                    // compute relative paths from generated contract dir to repo paths
                    let gen_dir: PathBuf = self.target.contract_dir();
                    let vendor_exec = repo_root.join("vendor").join("multiversx-chain-vm-executor");
                    let vendor_wasmer = repo_root.join("vendor").join("multiversx-chain-vm-executor-wasmer-experimental");
                    let chain_vm = repo_root.join("chain").join("vm");

                    let rel_exec = diff_paths(&vendor_exec, &gen_dir)
                        .unwrap_or(vendor_exec.clone())
                        .to_string_lossy()
                        .to_string();
                    let rel_wasmer = diff_paths(&vendor_wasmer, &gen_dir)
                        .unwrap_or(vendor_wasmer.clone())
                        .to_string_lossy()
                        .to_string();
                    let rel_chain_vm = diff_paths(&chain_vm, &gen_dir)
                        .unwrap_or(chain_vm.clone())
                        .to_string_lossy()
                        .to_string();

                    // build toml structure: [patch.crates-io]
                    let mut crates_io: TomlTable = TomlTable::new();

                    let mut exec_entry: TomlTable = TomlTable::new();
                    exec_entry.insert("path".to_string(), TomlValue::String(rel_exec));
                    crates_io.insert(
                        "multiversx-chain-vm-executor".to_string(),
                        TomlValue::Table(exec_entry),
                    );

                    let mut wasmer_entry: TomlTable = TomlTable::new();
                    wasmer_entry.insert("path".to_string(), TomlValue::String(rel_wasmer));
                    crates_io.insert(
                        "multiversx-chain-vm-executor-wasmer-experimental".to_string(),
                        TomlValue::Table(wasmer_entry),
                    );

                    let mut chain_entry: TomlTable = TomlTable::new();
                    chain_entry.insert("path".to_string(), TomlValue::String(rel_chain_vm));
                    crates_io.insert("multiversx-chain-vm".to_string(), TomlValue::Table(chain_entry));

                    let mut patch_tbl: TomlTable = TomlTable::new();
                    patch_tbl.insert("crates-io".to_string(), TomlValue::Table(crates_io));

                    toml.toml_value.insert("patch".to_string(), TomlValue::Table(patch_tbl));
                    toml.save_to_file(&cargo_toml_path);
                }
                _ => {}
            }
        }

    pub fn copy_template(&self, args_tag: FrameworkVersion) {
        self.template_source
            .copy_template(self.target.contract_dir(), args_tag);
    }

    pub fn update_dependencies(&self, args_tag: FrameworkVersion) {
        self.adjuster.update_cargo_toml_files(args_tag);
    }

    pub fn rename_template(&self) {
        self.adjuster.rename_template_to();
    }
}
