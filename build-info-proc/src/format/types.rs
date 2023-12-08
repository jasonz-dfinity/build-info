#![allow(dead_code)]

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum Type {
	Bool,
	Char,
	Integer,
	String,

	BuildInfo,
	OptimizationLevel,
	CrateInfo,
	CompilerInfo,
	CompilerChannel,
	VersionControl,
	GitInfo,

	Version,

	Option,
	Vec,
}

use std::fmt;
impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Type::Bool => write!(f, "bool"),
			Type::Char => write!(f, "char"),
			Type::Integer => write!(f, "integer"),
			Type::String => write!(f, "string"),

			Type::BuildInfo => write!(f, "build_info::BuildInfo"),
			Type::OptimizationLevel => write!(f, "build_info::OptimizationLevel"),
			Type::CrateInfo => write!(f, "build_info::CrateInfo"),
			Type::CompilerInfo => write!(f, "build_info::CompilerInfo"),
			Type::CompilerChannel => write!(f, "build_info::CompilerChannel"),
			Type::VersionControl => write!(f, "build_info::VersionControl"),
			Type::GitInfo => write!(f, "build_info::GitInfo"),

			Type::Version => write!(f, "build_info::semver::Version"),

			Type::Option => write!(f, "Option<_>"),
			Type::Vec => write!(f, "Vec<_>"),
		}
	}
}
