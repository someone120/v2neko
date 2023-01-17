use std::{
    io::{BufReader, Read},
    process::{Child, Command, Stdio},
};

use crate::{error::CoreConfigError, proxy::ProxyTrait};

pub struct Core {
    path: String,
    child: Option<Child>,
}

pub fn init(path: &str) -> Core {
    Core {
        path: path.to_owned(),
        child: None,
    }
}

impl ProxyTrait for Core {
    /// Restart the
    fn restart(self: &mut Core) {
        if let Some(_) = &self.child {
            Self::stop(self);
        }
        Self::start(self);
    }
    // fn start(&mut self) -> Child {
    //     Command::new(&self.path)
    //         .args(["-config", "connection.json"])
    //         .stdout(Stdio::piped())
    //         .spawn()
    //         .unwrap()

    // }

    fn start(&mut self) {
        let child = Command::new(&self.path)
            .args(["-config", "connection.json"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        self.child = Some(child);
    }

    fn stop(&mut self) {
        if let Some(i) = &mut self.child {
            i.kill().unwrap();
        }
    }
    fn check_version(&self) -> Result<String, CoreConfigError> {
        let output = Command::new(&self.path).arg("-version").output();
        match output {
            Ok(o) => Ok(String::from_utf8(o.stdout).unwrap()),
            Err(_) => Err(CoreConfigError {
                msg: "failed to run command".to_string(),
                code: -1,
            }),
        }
    }

    fn poll_output(&mut self) -> Option<String> {
        let stdout = self.child.as_mut().unwrap().stdout.as_mut().unwrap();
        let mut buf = BufReader::new(stdout);
        let mut vec: Vec<u8> = Vec::new();
        match buf.read_to_end(&mut vec) {
            Ok(_) => Some(String::from_utf8(vec).unwrap()),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::thread;

    use super::*;
    #[test]
    fn test_check_version() {
        let core = init("/usr/bin/xray");
        let output = core.check_version().unwrap();

        assert_eq!(
            output,
            String::from(
                r#"Xray 1.7.2 (Xray, Penetrates Everything.) Custom (go1.19.4 linux/amd64)
A unified platform for anti-censorship.
"#
            )
        );
    }

    #[test]
    fn test_check_version_err() {
        let core = init("");
        let output = core.check_version();

        assert!(output.is_err());
    }

    #[tokio::test]
    async fn test_start() {
        let mut core = init("/usr/bin/xray");
        core.start();
        thread::sleep(time::Duration::from_secs(1));
        tokio::spawn(async {
            thread::sleep(time::Duration::from_secs(1));
            assert!(false, "timeout");
        });
    }
}
