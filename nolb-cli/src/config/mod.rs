pub(crate) mod file_format;

use serde::{Deserialize, Serialize};

// May be reworked as proc_macro under struct, like
// #[defaults_for_serde(timeout=10)]
macro generate_default_for_serde($func_name:tt, $type: ty, $default_value: tt) {
    fn $func_name() -> $type {
        $default_value
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigRoot {
    pub agents: Option<Vec<agent::Agent>>,
    pub applications: Option<Vec<application::Application>>,
}

pub mod agent {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use validator::Validate;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Agent {
        pub name: String,
        pub provider: String,
        pub provider_properties: HashMap<String, crate::parsing::unified_parsed_value::Value>,
        pub server_auth: Option<Auth>,
        pub hardware_requirements: HardwareRequirements,
        pub autoscaling_policy: Option<AutoscalingPolicy>,
        pub applications: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AuthParameter {
        Password(String),
        PubKey(String),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
    pub struct Auth {
        #[serde(default = "ssh_port_default")]
        pub ssh_port: u64, // todo: replace with lesser digit and additional check for valid unix port number
        pub user: Option<String>,
        #[serde(flatten)]
        pub auth_parameter: Option<AuthParameter>,
    }

    super::generate_default_for_serde!(ssh_port_default, u64, 22);

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HardwareRequirements {
        pub cpu: u64,
        pub memory: u64,
        pub swap_size: Option<i64>,
        pub disk_volume: u64,
        #[serde(default)]
        pub disk_volume_spread_over: u64,
        #[serde(default)]
        pub disk_volume_spread_above: u64,
        pub disk_type: Option<DiskType>,
        #[serde(default)]
        pub dist_type_hard_requirement: bool,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum DiskType {
        Hdd,
        Ssd,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct AutoscalingPolicy {
        pub cpu_load: Option<u8>,
        pub memory_load: Option<u8>,
        pub swap_load: Option<u8>,
        #[serde(default)]
        pub factors: Factors,
        #[serde(default)]
        pub max_instances: Option<u64>,
        #[serde(default = "min_instances_default")]
        pub min_instances: u64,
    }

    super::generate_default_for_serde!(min_instances_default, u64, 1);

    #[derive(Debug, Clone, PartialEq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
    #[repr(u8)]
    pub enum Factors {
        One = 1,
        Two = 2,
        Three = 3,
    }

    impl From<Factors> for u8 {
        fn from(factors: Factors) -> Self {
            match factors {
                Factors::One => Factors::One as u8,
                Factors::Two => Factors::One as u8,
                Factors::Three => Factors::Three as u8,
            }
        }
    }

    impl Default for Factors {
        fn default() -> Self {
            Factors::One
        }
    }
}

pub mod application {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Application {
        pub name: String,
        #[serde(flatten)]
        pub platform: Platform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Platform {
        Docker(Docker),
        Host(Host),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Docker {
        pub image: String,
        #[serde(default = "image_pull_timeout_default")]
        pub image_pull_timeout: u64,
        #[serde(default = "timeout_default")]
        pub timeout: u64,
        #[serde(flatten)]
        pub docker_run: DockerRun,
    }

    super::generate_default_for_serde!(image_pull_timeout_default, u64, 10);
    super::generate_default_for_serde!(timeout_default, u64, 10);

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum DockerRun {
        RunCmd(String),
        Compose(String),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Host {
        pub distribution: Distribution,
        pub packages: Option<Packages>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Distribution {
        pub distributor_id: String,
        pub release: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Packages {
        #[serde(default)]
        pub update: bool,
        #[serde(default)]
        pub upgrade: bool,
        pub install: Option<Vec<String>>,
        pub post_install_commands: Option<Vec<String>>,
    }
}

// #[cfg(tests)]
mod tests {
    #[allow(unused_imports)]
    use crate::utility::macros::read_file_content;

    #[test]
    fn test_full_yaml() {
        let source = read_file_content!("./sample_configs/nolb.yaml");
        let _ =
            serde_yaml::from_str::<crate::config::ConfigRoot>(source.as_str()).expect("Unable to parse ConfigRoot!");
    }

    #[test]
    fn test_full_json() {
        let source = read_file_content!("./sample_configs/nolb.json");
        let _ =
            serde_json::from_str::<crate::config::ConfigRoot>(source.as_str()).expect("Unable to parse ConfigRoot!");
    }

    #[test]
    fn test_full_toml() {
        let source = read_file_content!("./sample_configs/nolb.toml");
        let _ = toml::from_str::<crate::config::ConfigRoot>(source.as_str()).expect("Unable to parse ConfigRoot!");
    }

    #[test]
    fn test_only_required_fields() {
        let source = read_file_content!("./sample_configs/only_required_fields.yaml");
        let _ =
            serde_yaml::from_str::<crate::config::ConfigRoot>(source.as_str()).expect("Unable to parse ConfigRoot!");
    }
}
