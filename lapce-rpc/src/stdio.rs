use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::{self, BufRead, Write},
    thread,
};

pub fn stdio_transport<W, R, REQ, RSP>(
    mut writer: W,
    writer_receiver: Receiver<REQ>,
    mut reader: R,
    reader_sender: Sender<RSP>,
) where
    W: 'static + Write + Send,
    R: 'static + BufRead + Send,
    REQ: 'static + Serialize + Send,
    RSP: 'static + DeserializeOwned + Send + Sync,
{
    thread::spawn(move || -> Result<()> {
        writer_receiver
            .into_iter()
            .try_for_each(|it| write_msg(&mut writer, &it))?;
        Ok(())
    });
    thread::spawn(move || -> Result<()> {
        loop {
            let msg: RSP = read_msg(&mut reader)?;
            reader_sender.send(msg)?;
        }
    });
}

fn write_msg<W>(out: &mut W, msg: impl Serialize) -> io::Result<()>
where
    W: Write,
{
    let msg = format!("{}\n", serde_json::to_string(&msg)?);
    out.write_all(msg.as_bytes())?;
    out.flush()?;
    Ok(())
}

fn read_msg<RSP: DeserializeOwned>(inp: &mut impl BufRead) -> io::Result<RSP> {
    let mut buf = String::new();
    let _s = inp.read_line(&mut buf)?;
    let response = serde_json::from_str(&buf)?;
    Ok(response)
}

#[allow(dead_code)]
pub(crate) fn make_io_threads(
    reader: thread::JoinHandle<io::Result<()>>,
    writer: thread::JoinHandle<io::Result<()>>,
) -> IoThreads {
    IoThreads { reader, writer }
}

pub struct IoThreads {
    #[allow(dead_code)]
    reader: thread::JoinHandle<io::Result<()>>,

    #[allow(dead_code)]
    writer: thread::JoinHandle<io::Result<()>>,
}

impl IoThreads {
    #[allow(dead_code)]
    pub fn join(self) -> io::Result<()> {
        match self.reader.join() {
            Ok(r) => r?,
            Err(err) => {
                panic!("{:?}", err);
            }
        }
        match self.writer.join() {
            Ok(r) => r,
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}
