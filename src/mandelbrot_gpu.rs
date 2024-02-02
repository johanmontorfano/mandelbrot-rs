use std::borrow::Cow;
use std::mem::size_of_val;
use std::rc::Rc;
use softbuffer::Buffer;
use wgpu::{Adapter, BindGroupDescriptor, BindGroupEntry, BufferAddress, BufferDescriptor, BufferUsages, ComputePipelineDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, Queue, RequestAdapterOptions, ShaderModule, ShaderModuleDescriptor, ShaderSource};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;
use crate::mandelbrot::Mandelbrot;

// This struct is initialized for the whole generation process and stores access to the GPU, shaders
// to use, ... It should be initialized early in the app's lifetime.
pub struct GPUInstanceWithShader {
    device: Device,
    queue: Queue,
    shader: ShaderModule
}

impl GPUInstanceWithShader {
    pub async fn initialize() -> Option<Self> {
        let instance = Instance::default();
        let adapter = instance.request_adapter(&RequestAdapterOptions::default()).await?;

        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::downlevel_defaults()
        }, None).await.unwrap();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None, source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("mandelbrot.wgsl")))
        });

        Some(Self { device, queue, shader })
    }
}

impl Mandelbrot {
    // Instead of computing only one pixel color from a coordinate and letting all the pixels and
    // buffer management to the caller, this function manages a referenced `softbuffer::Buffer` and
    // automatically push computed pixels to this buffer.
    // It means that the outer function only have to generate a Mandelbrot context and call this
    // function with a shader configuration to have it fully rendered using the GPU.
    pub fn get_color_buffer_with_shader(
        &self,
        buffer: &Buffer<Rc<Window>, Rc<Window>>,
        shader: GPUInstanceWithShader
    ) -> Vec<u8> {
        // Initializes data buffers and initializes GPU session with standard values.

        [0, 0, 0]
    }
}
