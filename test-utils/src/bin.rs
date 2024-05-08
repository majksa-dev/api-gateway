use std::{env, path, process::Command};

fn target_dir() -> path::PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap()
}

fn cargo_bin<S: AsRef<str>>(name: S) -> path::PathBuf {
    let env_var = format!("CARGO_BIN_EXE_{}", name.as_ref());
    std::env::var_os(env_var)
        .map(|p| p.into())
        .unwrap_or_else(|| {
            target_dir().join(format!("{}{}", name.as_ref(), env::consts::EXE_SUFFIX))
        })
}

pub fn server_cmd() -> Command {
    let bin = cargo_bin("api-gateway");
    Command::new(bin)
}
