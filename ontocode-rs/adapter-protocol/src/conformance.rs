use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::Duration;
use anyhow::{Context, Result};
use crate::{AdapterMessage, ProtocolParser};

pub struct ConformanceRunner {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    parser: ProtocolParser,
}

impl ConformanceRunner {
    pub fn spawn(cmd: &str, args: &[&str], max_frame_bytes: usize) -> Result<Self> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("failed to spawn adapter process")?;

        let stdin = child.stdin.take().expect("failed to open stdin");
        let stdout = BufReader::new(child.stdout.take().expect("failed to open stdout"));

        Ok(Self {
            child,
            stdin,
            stdout,
            parser: ProtocolParser::new(max_frame_bytes),
        })
    }

    pub fn send(&mut self, msg: &AdapterMessage) -> Result<()> {
        let json = self.parser.serialize(msg)?;
        writeln!(self.stdin, "{}", json)?;
        self.stdin.flush()?;
        Ok(())
    }

    pub fn recv(&mut self, timeout: Duration) -> Result<AdapterMessage> {
        // Simple synchronous read for conformance testing
        let mut line = String::new();
        // In a real implementation we'd use tokio with timeout,
        // but for a simple runner we can set read timeout on the pipe if supported,
        // or just rely on the test runner timeout.
        self.stdout.read_line(&mut line)?;
        if line.is_empty() {
            anyhow::bail!("adapter process closed stdout unexpectedly");
        }
        self.parser.parse_line(line.trim())
    }

    pub fn stop(mut self) -> Result<()> {
        let _ = self.child.kill();
        Ok(())
    }
}
