use crate::*;
use os_pipe::{pipe, PipeReader, PipeWriter};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, ExitStatus};
use std::thread;

pub struct ScServerProcess {
    child: Child,
    pipe_reader: PipeReader,
    #[allow(dead_code)]
    pipe_writer: PipeWriter,
}

impl ScServerProcess {
    pub fn new(options: &Options) -> ScClientResult<Self> {
        let (pipe_reader, pipe_writer) = pipe()?;
        let child = match Command::new(options.path.clone())
            .args(options.to_args())
            .stdout(pipe_writer.try_clone().unwrap())
            .stderr(pipe_writer.try_clone().unwrap())
            .spawn()
        {
            Err(e) => panic!("couldn't start {}: {}", options.path, e),
            Ok(process) => process,
        };
        let process = ScServerProcess {
            child,
            pipe_reader,
            pipe_writer,
        };
        process.guess_ready_and_pipe_stdout()?;
        Ok(process)
    }

    fn guess_ready_and_pipe_stdout(&self) -> ScClientResult<()> {
        let pipe_reader = self.pipe_reader.try_clone()?;
        let current_thread = thread::current();

        thread::spawn(move || {
            let mut child_out = BufReader::new(pipe_reader);
            let mut line = String::new();
            loop {
                child_out.read_line(&mut line).unwrap();
                print!("{}", line);
                if line.contains("ready") {
                    current_thread.unpark()
                }
                line.clear();
                thread::sleep(std::time::Duration::from_millis(1));
            }
        });

        Ok(thread::park())
    }

    pub fn kill_child(&mut self) -> ScClientResult<()> {
        Ok(self.child.kill()?)
    }

    pub fn wait_for_finish(&mut self) -> ScClientResult<ExitStatus> {
        Ok(self.child.wait()?)
    }
}
