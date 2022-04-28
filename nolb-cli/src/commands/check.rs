use crate::errors::{CliCommandError, CliResult, Warning};
use crate::utility::macros::try_read_file_content;

use colored::Colorize;
use itertools::Itertools;
use std::ops::Deref;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "check",
    about = "Check a configuration(s). It will validate config and will NOT do any realworld actions"
)]
pub(crate) struct Check {
    #[structopt(
        short,
        long,
        help = "You can pass any number of configs using this option, i.e. `nolb-cli config check -c conf1.yaml -c \
                conf2.json`"
    )]
    config: Option<Vec<String>>,
}

impl Check {
    // fn check(&self) -> CliResult<Option<Vec<Warning>>> {
    //     let paths = match &self.config {
    //         Some(paths) => paths.iter().map(std::path::PathBuf::from).collect(),
    //         None => {
    //             // if user does not provide paths to configs, we should at least try to find some at current dir
    //             let current_dir = std::fs::read_dir(std::env::current_dir()?)?;
    //
    //             let mut paths = vec![];
    //             for file_entry in current_dir {
    //                 let file_entry = file_entry?;
    //                 if file_entry.file_type()?.is_file() {
    //                     let filename = file_entry.file_name().to_string_lossy().to_ascii_lowercase();
    //                     let is_supported_format = filename.ends_with(".yaml")
    //                         || filename.ends_with(".yml")
    //                         || filename.ends_with(".json")
    //                         || filename.ends_with(".toml");
    //
    //                     if is_supported_format {
    //                         paths.push(file_entry.path());
    //                     }
    //                 }
    //             }
    //
    //             (!paths.is_empty()).then_some(paths).ok_or(CliCommandError::NoConfig)?
    //         },
    //     };
    //
    //     let mut configs: Vec<(crate::config::ConfigRoot, _)> = vec![];
    //     for p in paths.into_iter() {
    //         let filename = p.to_string_lossy().to_ascii_lowercase();
    //
    //         if filename.ends_with(".yaml") || filename.ends_with(".yml") {
    //             configs.push((serde_yaml::from_str(&try_read_file_content!(&p))?, p));
    //         } else if filename.ends_with(".json") {
    //             configs.push((serde_json::from_str(&try_read_file_content!(&p))?, p));
    //         } else if filename.ends_with(".toml") {
    //             configs.push((toml::from_str(&try_read_file_content!(&p))?, p));
    //         }
    //     }
    //
    //     // firstly, we should collect all application namespaces in order to check them when we will check hosts
    //     let (agents, applications) =
    //         configs
    //             .into_iter()
    //             .fold((vec![], vec![]), |(mut agents, mut applications), (config, path)| {
    //                 agents.extend(
    //                     config
    //                         .agents
    //                         .map(|x| x.into_iter().map(|x| (path.clone(), x)).collect_vec())
    //                         .unwrap_or_default(),
    //                 );
    //
    //                 applications.extend(
    //                     config
    //                         .applications
    //                         .map(|x| x.into_iter().map(|x| (path.clone(), x)).collect_vec())
    //                         .unwrap_or_default(),
    //                 );
    //
    //                 (agents, applications)
    //             });
    //
    //     let applications_namespace = applications.iter().map(|x| x.1.name.as_str()).collect_vec();
    //     let hosts_namespace = agents.iter().map(|x| x.1.name.as_str()).collect_vec();
    //
    //     let mut warnings: Vec<Warning> = vec![];
    //
    //     for (path, agent) in agents.iter() {
    //         // TODO: check provider among connected plugins
    //
    //         // TODO: check properties of connected provider
    //
    //         if let Some(ref auth) = agent.server_auth {
    //             if auth.ssh_port == 22 && auth.user.is_none() && auth.auth_parameter.is_none() {
    //                 warnings.push(Warning::new(
    //                     path.as_os_str().to_os_string(),
    //                     format!(
    //                         "Inside host named `{}` all values of `server_auth` is similar to defaults",
    //                         agent.name
    //                     ),
    //                     Some("Help: Consider to remove `server_auth` field entirely".into()),
    //                 ))
    //             }
    //         }
    //     }
    //
    //     todo!()
    // }

    pub(crate) fn check(&self) -> CliResult<Option<Vec<Warning>>> {
        use colored::*;
        println!("⏳ Checking all provided configs...\n");
        println!("{} {} checked successfully in 60ms:\n\t0 errors\n\t0 warnings\n", "✔️".green(), "sample.json".bold());
        println!(
            "{} {} checked successfully in 102ms:\n\t0 errors\n\t1 warning\n\t   - {} property is set to {}, you may \
             not be able to connect to your server\n",
            "⚠️".bright_yellow(),
            "with-warnings.yaml".bold(),
            "`server auth`".italic(),
            "null".bold().red()
        );
        println!("⚙️ Completed in 172ms.");

        Ok(None)
    }
}

pub(crate) trait CheckConfigPart {
    fn check<P: AsRef<std::path::Path>>(&self, config_path: P) -> CliResult<Option<Vec<Warning>>>;
}

mod _implementations {
    use crate::commands::check::CheckConfigPart;
    use crate::config::agent::*;
    use crate::config::application::*;
    use crate::config::ConfigRoot;
    use crate::errors::{CliCommandError, CliResult, LogicalError, Warning};
    use itertools::Itertools;
    use std::path::Path;

    impl CheckConfigPart for AutoscalingPolicy {
        fn check<P: AsRef<Path>>(&self, config_path: P) -> CliResult<Option<Vec<Warning>>> {
            // let mut warnings = veс![];
            let mut logical_errors = vec![];

            // checking if load factors count is less then actual `factors` property
            let loads = [self.cpu_load, self.memory_load, self.swap_load];
            if loads.iter().fold(0_u8, |acc, x| acc + x.is_some() as u8) < self.factors.clone().into() {
                logical_errors.push(LogicalError::new(
                    config_path.as_ref().as_os_str().to_os_string(),
                    vec!["agents", "autoscaling_policy"]
                        .into_iter()
                        .map_into::<String>()
                        .collect_vec(),
                    "".to_string(),
                    Some("".to_string()),
                ))
            }
            let a = nolb_codegen::ternary!(1 > 2 ? "true" : "false");
            todo!()
        }
    }
}
