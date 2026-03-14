use cpal::{Device, default_host, traits::HostTrait};

use crate::error::{SttError, SttResult};

pub(crate) fn start_record() -> SttResult {
    let mic = default_input_device()?;
    
}

fn default_input_device() -> SttResult<Device> {
    default_host()
        .default_input_device()
        .ok_or(SttError::MissingInputDevice)
}
