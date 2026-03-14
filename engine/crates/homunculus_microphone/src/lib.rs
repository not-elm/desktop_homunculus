mod error;
mod microphone;
mod stt;

use serde::{Deserialize, Serialize};
use std::sync::mpsc::Receiver;

pub struct MicrophoneSession {
    pub rx: Receiver<MicrophoneSentence>,
    pub tx: Receiver<MicrophoneSentence>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MicrophoneSentence {
    pub text: String,
    pub timestamp: f64,
    pub language: String,
}
