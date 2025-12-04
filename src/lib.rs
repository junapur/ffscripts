use anyhow::{Context, Result, bail};
use std::{
    io::ErrorKind::NotFound,
    process::{Command, Stdio},
};

pub fn check_dependencies() -> Result<()> {
    for tool in ["ffmpeg", "ffprobe"] {
        let result = Command::new(tool)
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match result {
            Ok(_) => continue,

            Err(e) => match e.kind() {
                NotFound => bail!(
                    "{} was not found. Please visit https://ffmpeg.org/download.html.",
                    tool
                ),

                _ => return Err(e).context(format!("Failed to check if {} is installed.", tool)),
            },
        }
    }

    Ok(())
}
