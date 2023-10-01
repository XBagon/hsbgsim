use std::{
    env,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let args = env::args().skip(1);
    let staged_files: Vec<String> = args.collect();

    println!("> Checking staged files");
    let mut codegen_files = 0;
    for changed_file in staged_files {
        if changed_file == "src/minions/variants/mod.rs" {
            codegen_files += 1;
        } else if changed_file == "hsbgsim_codegen/src/main.rs" {
            codegen_files += 2;
        } else if changed_file.ends_with(".new") {
            println!(
                "Don't commit `.new` snapshots. Review/accept them before with `cargo insta`."
            );
            return ExitCode::FAILURE;
        }
    }
    if codegen_files == 1 {
        println!("Just generated file was changed.");
        return ExitCode::FAILURE;
    } else {
        println!("> cargo run -p hsbgsim_codegen");
        let codegen = Command::new("cargo").args(["run", "-p", "hsbgsim_codegen"]).output();
        match codegen {
            Ok(status) if status.status.success() => {}
            _ => return ExitCode::FAILURE,
        }
    }

    println!("> cargo fmt");
    let fmt = Command::new("cargo").args(["fmt"]).output();
    match fmt {
        Ok(status) if status.status.success() => {}
        _ => return ExitCode::FAILURE,
    }
    println!("> cargo clippy");
    let clippy = Command::new("cargo").args(["clippy"]).output();
    match clippy {
        Ok(status) if status.status.success() => {}
        _ => return ExitCode::FAILURE,
    }
    println!("> cargo test");
    let test = Command::new("cargo").args(["test"]).output();
    match test {
        Ok(status) if status.status.success() => {}
        _ => return ExitCode::FAILURE,
    }

    ExitCode::SUCCESS
}
