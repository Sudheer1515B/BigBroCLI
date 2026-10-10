use anyhow::Context;
use anyhow::Result;
use std::io::Write;
use std::process::Child;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

pub struct SshConnection {
    child: Child,
}

impl SshConnection {
    pub fn connect(target: &str) -> Result<(Self, ChildStdout)> {
        let child = Command::new("ssh")
            //starts ssh
            .arg("-T")
            .arg("-o")
            .arg("ConnectTimeout=5")
            .arg(target)
            .arg("sh -s")
            .stdin(Stdio::piped())
            //for acces to the connection's in and out
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to start SSH")?;

        let mut connection = Self { child };

        //shell reads script from stdin

        let mut stdin = connection
            .child
            .stdin
            .take()
            .context("Couldn't access SSH input")?;

        let script = include_str!("../scripts/collect.sh");
        //using the previously made script, this macro puts that
        // in the binary when compiling
        stdin.write_all(script.as_bytes())?;

        //script is sent but stdout is kept open

        drop(stdin);

        let stdout = connection
            .child
            .stdout
            .take()
            .context("Couldn't access SSH stdout")?;
        Ok((connection, stdout))
    }
}

impl Drop for SshConnection {
    fn drop(&mut self) {
        //close ssh after monitor exits
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
