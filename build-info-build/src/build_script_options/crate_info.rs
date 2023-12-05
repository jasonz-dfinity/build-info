use build_info_common::CrateInfo;
use cargo_metadata::*;

impl crate::BuildScriptOptions {
	/// Enables and disables dependency collection.
	///
	/// Dependency data is fairly large, which may cause problems, mainly by crashing the build process. If the project
	/// compiles successfully with dependency collection enabled, you are probably fine.
	pub fn collect_dependencies(mut self, collect_dependencies: bool) -> Self {
		self.collect_dependencies = collect_dependencies;
		self
	}
}

pub(crate) struct Manifest {
	pub crate_info: CrateInfo,
	pub workspace_root: String,
}

pub(crate) fn read_manifest() -> Manifest {
	let meta = MetadataCommand::new()
		.cargo_path(std::env::var_os("CARGO").expect("Cargo should exist"))
		.manifest_path(&*super::CARGO_TOML)
		.features(CargoOpt::NoDefaultFeatures)
		.exec()
		.unwrap();

	let mut enabled_features = vec![];
	for (key, _) in std::env::vars() {
		if let Some(p) = key.strip_prefix("CARGO_FEATURE_") {
			enabled_features.push(p.to_ascii_lowercase());
		}
	}

	let crate_info = CrateInfo {
		name: std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME"),
		version: std::env::var("CARGO_PKG_VERSION")
			.expect("CARGO_PKG_VERSION")
			.parse()
			.expect("CARGO_PKG_VERSION: parse"),
		authors: std::env::var("CARGO_PKG_AUTHORS").map_or_else(
			|_| Vec::new(),
			|authors| authors.split(':').map(|x| x.to_string()).collect::<Vec<_>>(),
		),
		enabled_features,
		available_features: Default::default(),
		dependencies: Default::default(),
		license: std::env::var("CARGO_PKG_LICENSE").ok(),
	};

	Manifest {
		crate_info,
		workspace_root: meta.workspace_root.into(),
	}
}
