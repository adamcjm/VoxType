use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub channel_count: u16,
}

/// List available audio input devices
pub fn list_input_devices() -> Vec<AudioDeviceInfo> {
    let host = cpal::default_host();
    let default_name = host
        .default_input_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    match host.input_devices() {
        Ok(devices) => devices
            .filter_map(|device| {
                let name = device.name().unwrap_or_else(|_| "Unknown".into());
                let is_default = name == default_name;
                let channel_count = device
                    .default_input_config()
                    .map(|c| c.channels())
                    .unwrap_or(0);
                Some(AudioDeviceInfo {
                    name,
                    is_default,
                    channel_count,
                })
            })
            .collect(),
        Err(_) => vec![],
    }
}
