use std::sync::mpsc;
use std::io;

use crate::server::BcMsg;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MsgSend(mpsc::SendError<BcMsg>)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<mpsc::SendError<BcMsg>> for Error {
    fn from(e: mpsc::SendError<BcMsg>) -> Self {
        Error::MsgSend(e)
    }
}

