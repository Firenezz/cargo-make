//! # installer
//!
//! Installs depedencies.
//!

use command;
use log::Log;
use std::process::Command;
use types::Task;

fn is_crate_installed(
    logger: &Log,
    crate_name: &str,
) -> bool {
    logger.verbose::<()>("Getting list of installed cargo commands.", &[], None);
    let result = Command::new("cargo").arg("--list").output();

    match result {
        Ok(output) => {
            let mut found = false;

            command::validate_exit_code(Ok(output.status));

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split(' ').collect();
            for mut line in lines {
                line = line.trim();

                logger.verbose::<()>("Checking: ", &[&line], None);

                if line.contains(crate_name) && crate_name.contains(line) {
                    found = true;
                    logger.verbose::<()>("Found installed crate.", &[], None);

                    break;

                }
            }

            found
        }
        Err(error) => {
            logger.error("Unable to check if crate is installed: ", &[crate_name], Some(&error));
            false
        }
    }
}

pub fn install(
    logger: &Log,
    task_config: &Task,
) {
    match task_config.install_crate {
        Some(ref crate_name) => {
            let cargo_command = match task_config.args {
                Some(ref args) => &args[0],
                None => {
                    logger.error::<()>("Missing cargo command to invoke.", &[], None);
                    panic!("Missing cargo command to invoke.");
                }
            };

            if !is_crate_installed(&logger, cargo_command) {
                command::run_command(&logger, "cargo", &Some(vec!["install".to_string(), crate_name.to_string()]));
            }
        }
        None => {
            match task_config.install_script {
                Some(ref script) => command::run_script(&logger, &script),
                None => logger.verbose::<()>("No installation script defined.", &[], None),
            }
        }
    }
}