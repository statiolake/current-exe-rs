pub use to_absolute::Result;

use to_absolute::to_absolute;

use std::env;
use std::fs;
use std::path::PathBuf;

pub fn current_exe() -> Result<PathBuf> {
    let current_exe = env::current_exe()?;
    match fs::read_link(&current_exe) {
        Err(_) => Ok(current_exe),
        Ok(actual_exe) => to_absolute(
            current_exe
                .parent()
                .expect("FILE must have the parent directory!"),
            actual_exe,
        ),
    }
}
