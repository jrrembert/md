use cocoa::{appkit::NSView, base::id as cocoa_id};
use metal::{Device, MTLPixelFormat, MetalLayer};
use objc::{msg_send, sel, sel_impl};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowExtMacOS,
    window::WindowBuilder,
};

struct MetalRenderer {
    device: Device,
    layer: MetalLayer,
}

impl MetalRenderer {
    fn new(window: &winit::window::Window) -> Self {
        // Create Metal device
        let device = Device::system_default().expect("No Metal device found");
        
        // Create Metal layer
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        // Get the CGRect of the window's view
        unsafe {
            let view = window.ns_view() as cocoa_id;
            let layer_ptr: *mut MetalLayer = &layer as *const _ as *mut _;
            let () = msg_send![view, setLayer: layer_ptr];
            let () = msg_send![view, setWantsLayer: true];
        }

        Self { device, layer }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    
    let window = WindowBuilder::new()
        .with_title("Markdown Editor")
        .with_inner_size(winit::dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)
        .unwrap();

    let _renderer = MetalRenderer::new(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    }).unwrap();
} 