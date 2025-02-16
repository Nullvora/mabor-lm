use spinners::{Spinner, Spinners};
use std::process::{exit, Command};

use yansi::Paint;

const BURNLM_SUPERVISER_RESTART_EXIT_CODE: i32 = 8;
const BURNLM_BACKEND_ENVVAR: &str = "BURNLM_BACKEND";
const DEFAULT_BURN_BACKEND: &str = "wgpu";

fn main() {
    println!();
    // retrieve backend
    let mut exit_code = BURNLM_SUPERVISER_RESTART_EXIT_CODE;
    let mut backend = match std::env::var(BURNLM_BACKEND_ENVVAR) {
        Ok(backend) => backend,
        Err(_) => {
            let hint = format!(
                "╭──────────────────────────────────────────────────────────────────────────╮
│ 💡 Hint: No environment variable 'BURNLM_BACKEND' defined. Using default │
│ Burn backend which is '{}'. To get a list of all supported backends on │
│ this platform use 'cargo burnlm backends'.                               │
╰──────────────────────────────────────────────────────────────────────────╯
",
                DEFAULT_BURN_BACKEND
            );
            print!("{}", hint.bright_yellow().bold());
            DEFAULT_BURN_BACKEND.to_string()
        }
    };
    backend = backend.to_lowercase();
    // build and run arguments
    let inference_feature = format!("burnlm-inference/{}", backend);
    let common_args = vec![
        "--release",
        "--bin",
        "burnlm-cli",
        "--no-default-features",
        "--features",
        &inference_feature,
        "--quiet",
        "--color",
        "always",
    ];
    let mut build_args = vec!["build"];
    build_args.extend(common_args.clone());
    let mut run_args = vec!["run"];
    run_args.extend(common_args);
    run_args.push("--");
    let passed_args: Vec<String> = std::env::args().skip(1).collect();
    run_args.extend(passed_args.iter().map(|s| s.as_str()));

    // Rebuild and restart burnlm while its exit code is SUPERVISER_RESTART_EXIT_CODE
    while exit_code == BURNLM_SUPERVISER_RESTART_EXIT_CODE {
        let compile_msg = "compiling burnlm CLI, please wait...";
        let mut sp = Spinner::new(Spinners::Bounce, compile_msg.bright_black().to_string());
        // build burnlm cli
        let build_output = Command::new("cargo")
            .args(&build_args)
            .output()
            .expect("build command should compile burnlm successfully");
        // build step results
        let stderr_text = String::from_utf8_lossy(&build_output.stderr);
        if !stderr_text.is_empty() {
            println!("{stderr_text}");
        }
        if !build_output.status.success() {
            exit(build_output.status.code().unwrap_or(1));
        }
        // stop the spinner
        let completion_msg = format!(
            "{} {}",
            "✓".bright_green().bold(),
            "burnlm CLI ready!".bright_black().bold(),
        );
        sp.stop_with_message(completion_msg);
        // execute burnlm
        let run_status = Command::new("cargo")
            .env(BURNLM_BACKEND_ENVVAR, backend.clone())
            .args(&run_args)
            .status()
            .expect("burnlm command should execute successfully");
        exit_code = run_status.code().unwrap_or(1);
    }
    exit(exit_code);
}
