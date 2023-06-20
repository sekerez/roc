//! Provides types and helpers for compiler targets such as `default_x86_64`.
#![warn(clippy::dbg_macro)]
// See github.com/roc-lang/roc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant)]

use strum_macros::{EnumCount, EnumIter, IntoStaticStr};
use target_lexicon::Triple;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperatingSystem {
    Windows,
    Unix,
    Wasi,
}

impl OperatingSystem {
    pub const fn new(target: target_lexicon::OperatingSystem) -> Option<Self> {
        match target {
            target_lexicon::OperatingSystem::Windows => Some(OperatingSystem::Windows),
            target_lexicon::OperatingSystem::Wasi => Some(OperatingSystem::Wasi),
            target_lexicon::OperatingSystem::Linux => Some(OperatingSystem::Unix),
            target_lexicon::OperatingSystem::MacOSX { .. } => Some(OperatingSystem::Unix),
            target_lexicon::OperatingSystem::Darwin => Some(OperatingSystem::Unix),
            target_lexicon::OperatingSystem::Unknown => Some(OperatingSystem::Unix),
            _ => None,
        }
    }

    pub const fn object_file_ext(&self) -> &str {
        match self {
            OperatingSystem::Windows => "obj",
            OperatingSystem::Unix => "o",
            OperatingSystem::Wasi => "wasm",
        }
    }

    pub const fn executable_file_ext(&self) -> Option<&str> {
        match self {
            OperatingSystem::Windows => Some("exe"),
            OperatingSystem::Unix => None,
            OperatingSystem::Wasi => Some("wasm"),
        }
    }
}

impl From<target_lexicon::OperatingSystem> for OperatingSystem {
    fn from(target: target_lexicon::OperatingSystem) -> Self {
        Self::new(target)
            .unwrap_or_else(|| unreachable!("unsupported operating system {:?}", target))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetInfo {
    pub architecture: Architecture,
    pub operating_system: OperatingSystem,
}

impl TargetInfo {
    pub const fn ptr_width(&self) -> PtrWidth {
        self.architecture.ptr_width()
    }

    pub const fn ptr_size(&self) -> usize {
        match self.ptr_width() {
            PtrWidth::Bytes4 => 4,
            PtrWidth::Bytes8 => 8,
        }
    }

    pub const fn max_by_value_size(&self) -> usize {
        // Pass values larger than 4 machine words by reference.
        // This is a reasonable default for most architectures. We want to pass large values by
        // reference because it's more efficient than copying them around on the stack, and puts
        // less pressure on CPU registers.
        self.ptr_size() * 4
    }

    pub const fn ptr_alignment_bytes(&self) -> usize {
        self.architecture.ptr_alignment_bytes()
    }

    pub const fn default_aarch64() -> Self {
        TargetInfo {
            architecture: Architecture::Aarch64,
            operating_system: OperatingSystem::Unix,
        }
    }

    pub const fn default_x86_64() -> Self {
        TargetInfo {
            architecture: Architecture::X86_64,
            operating_system: OperatingSystem::Unix,
        }
    }

    pub const fn default_wasm32() -> Self {
        TargetInfo {
            architecture: Architecture::Wasm32,
            operating_system: OperatingSystem::Wasi,
        }
    }
}

impl From<&target_lexicon::Triple> for TargetInfo {
    fn from(triple: &target_lexicon::Triple) -> Self {
        let architecture = Architecture::from(triple.architecture);
        let operating_system = OperatingSystem::from(triple.operating_system);

        Self {
            architecture,
            operating_system,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtrWidth {
    Bytes4 = 4,
    Bytes8 = 8,
}

/// These should be sorted alphabetically!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, EnumCount)]
#[repr(u8)]
pub enum Architecture {
    Aarch32,
    Aarch64,
    Wasm32,
    X86_32,
    X86_64,
}

impl Architecture {
    pub const fn ptr_width(&self) -> PtrWidth {
        use Architecture::*;

        match self {
            X86_64 | Aarch64 => PtrWidth::Bytes8,
            X86_32 | Aarch32 | Wasm32 => PtrWidth::Bytes4,
        }
    }

    pub const fn ptr_alignment_bytes(&self) -> usize {
        self.ptr_width() as usize
    }
}

impl From<target_lexicon::Architecture> for Architecture {
    fn from(target: target_lexicon::Architecture) -> Self {
        match target {
            target_lexicon::Architecture::X86_64 => Architecture::X86_64,
            target_lexicon::Architecture::X86_32(_) => Architecture::X86_32,
            target_lexicon::Architecture::Aarch64(_) => Architecture::Aarch64,
            target_lexicon::Architecture::Arm(_) => Architecture::Aarch32,
            target_lexicon::Architecture::Wasm32 => Architecture::Wasm32,
            _ => unreachable!("unsupported architecture"),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter, IntoStaticStr, PartialEq, Eq, Default)]
pub enum Target {
    #[strum(serialize = "system")]
    #[default]
    System,
    #[strum(serialize = "linux-x86-32")]
    LinuxX32,
    #[strum(serialize = "linux-x86-64")]
    LinuxX64,
    #[strum(serialize = "linux-arm-64")]
    LinuxArm64,
    #[strum(serialize = "macos-x86-64")]
    MacX64,
    #[strum(serialize = "macos-arm-64")]
    MacArm64,
    #[strum(serialize = "windows-x86-32")]
    WinX32,
    #[strum(serialize = "windows-x86-64")]
    WinX64,
    #[strum(serialize = "windows-arm-64")]
    WinArm64,
    #[strum(serialize = "wasm-32")]
    Wasm32,
}

const MACOS: target_lexicon::OperatingSystem = target_lexicon::OperatingSystem::MacOSX {
    major: 12,
    minor: 0,
    patch: 0,
};

impl Target {
    pub fn to_triple(self) -> Triple {
        use target_lexicon::*;

        match self {
            Target::System => Triple::host(),
            Target::LinuxX32 => Triple {
                architecture: Architecture::X86_32(X86_32Architecture::I386),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Elf,
            },
            Target::LinuxX64 => Triple {
                architecture: Architecture::X86_64,
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Elf,
            },
            Target::LinuxArm64 => Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Elf,
            },
            Target::WinX32 => Triple {
                architecture: Architecture::X86_32(X86_32Architecture::I386),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Windows,
                environment: Environment::Gnu,
                binary_format: BinaryFormat::Coff,
            },
            Target::WinX64 => Triple {
                architecture: Architecture::X86_64,
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Windows,
                environment: Environment::Gnu,
                binary_format: BinaryFormat::Coff,
            },
            Target::WinArm64 => Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Windows,
                environment: Environment::Gnu,
                binary_format: BinaryFormat::Coff,
            },
            Target::MacX64 => Triple {
                architecture: Architecture::X86_64,
                vendor: Vendor::Apple,
                operating_system: MACOS,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Macho,
            },
            Target::MacArm64 => Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Apple,
                operating_system: MACOS,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Macho,
            },
            Target::Wasm32 => Triple {
                architecture: Architecture::Wasm32,
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Wasi,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Wasm,
            },
        }
    }
}

impl From<&Target> for Triple {
    fn from(target: &Target) -> Self {
        target.to_triple()
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(self))
    }
}

impl std::str::FromStr for Target {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "system" => Ok(Target::System),
            "linux-x86-32" => Ok(Target::LinuxX32),
            "linux-x86-64" => Ok(Target::LinuxX64),
            "linux-arm-64" => Ok(Target::LinuxArm64),
            "macos-x86-64" => Ok(Target::MacX64),
            "macos-arm-64" => Ok(Target::MacArm64),
            "windows-x86-64" => Ok(Target::WinX64),
            "wasm-32" => Ok(Target::Wasm32),
            _ => Err(format!("Roc does not know how to compile to {}", string)),
        }
    }
}

pub fn get_target_triple_str(target: &target_lexicon::Triple) -> Option<&'static str> {
    match target {
        target_lexicon::Triple {
            architecture: target_lexicon::Architecture::Wasm32,
            ..
        } => Some(Target::Wasm32.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Linux,
            architecture: target_lexicon::Architecture::X86_64,
            ..
        } => Some(Target::LinuxX64.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Linux,
            architecture: target_lexicon::Architecture::Aarch64(_),
            ..
        } => Some(Target::LinuxArm64.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Darwin,
            architecture: target_lexicon::Architecture::Aarch64(_),
            ..
        } => Some(Target::MacArm64.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Darwin,
            architecture: target_lexicon::Architecture::X86_64,
            ..
        } => Some(Target::MacX64.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Windows,
            architecture: target_lexicon::Architecture::X86_64,
            ..
        } => Some(Target::WinX64.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Windows,
            architecture: target_lexicon::Architecture::X86_32(_),
            ..
        } => Some(Target::WinX32.into()),
        target_lexicon::Triple {
            operating_system: target_lexicon::OperatingSystem::Windows,
            architecture: target_lexicon::Architecture::Aarch64(_),
            ..
        } => Some(Target::WinArm64.into()),
        _ => None,
    }
}
