use metal::{Device, CommandQueue, Buffer};
use std::error::Error;

pub struct MetalContext {
    device: Device,
    command_queue: CommandQueue,
}

impl MetalContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let device = Device::system_default()
            .ok_or("No Metal device found")?;
        
        let command_queue = device.new_command_queue();
        
        Ok(Self {
            device,
            command_queue,
        })
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn command_queue(&self) -> &CommandQueue {
        &self.command_queue
    }

    pub fn create_buffer<T>(&self, data: &[T]) -> Buffer
    where
        T: Copy,
    {
        self.device.new_buffer_with_data(
            data.as_ptr() as *const std::ffi::c_void,
            (data.len() * std::mem::size_of::<T>()) as u64,
            metal::MTLResourceOptions::StorageModeShared,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metal_context_creation() {
        let context = MetalContext::new();
        assert!(context.is_ok());
    }
} 