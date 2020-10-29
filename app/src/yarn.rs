use std::env;
use std::path::{Path, PathBuf};

pub fn find_yarn() -> Option<PathBuf> {
  let program = platform::get_binary_filename("node");

  locate_binary(program)
}

fn locate_binary<P: AsRef<Path>>(name: P) -> Option<PathBuf> {
  env::var_os("PATH")
    .and_then(|paths| {
      env::split_paths(&paths)
        .filter_map(|dir| {
          let full_path = dir.join(&name);
          if full_path.is_file() {
            Some(full_path)
          } else {
            None
          }
        })
        .next()
    })
}

#[cfg(windows)]
mod platform {
  use std::path::{PathBuf};

  pub fn get_binary_filename(path: &str) -> PathBuf {
    PathBuf::from(path.to_owned()).join(".exe")
  }
}

