use std::{io::{BufReader, BufWriter, Stdout}, process::{Command, Stdio, Child}};

use tokio::task::JoinHandle;

use crate::{error::CoreConfigError, proxy::ProxyTrait, ws::new_ws};

pub struct Core {
    path: String,
    task: Option<JoinHandle<()>>,
    child:  Option<Child>
}

pub fn init(path: &str) -> Core {
    Core {
        path: path.to_owned(),
        task: None,
        child: None
    }
}

impl ProxyTrait for Core {
    /// Restart the 
    fn restart(self: &mut Core) -> &JoinHandle<()> {
        if let Some(_) = &self.task {
            Self::stop(self);
        }
        self.task = Some(Self::start(self));
        &self.task.as_ref().unwrap()
    }
    fn start(&mut self) -> JoinHandle<()> {
        let output = Command::new(&self.path)
            .args(["-config", "connection.json"])
            .spawn()
            .unwrap();
        let mut stream = BufReader::new(output.stdout.unwrap());
        tokio::spawn(async move { new_ws(15611, &mut stream) })
    }

    fn stop(&mut self) {
        if let Some(i) = &self.task {
            i.abort();
        }
        if let Some(i) = &mut self.child{
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
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_start(){
        let core = init("/usr/bin/xray");
        
    }
}
