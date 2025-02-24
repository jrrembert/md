use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::RenderPassColorAttachment;
use glyph_brush::{GlyphBrushBuilder, Section, Text};
use ab_glyph::FontArc;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let instance = wgpu::Instance::new(wgpu::Backends::METAL);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap();
    let size = window.inner_size();
    let swap_chain_format = surface.get_supported_formats(&adapter)[0];

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swap_chain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    surface.configure(&device, &config);

    // Load a font
    let font = FontArc::try_from_slice(include_bytes!("/System/Library/Fonts/Supplemental/Arial Unicode.ttf")).unwrap();
    let mut glyph_brush: glyph_brush::GlyphBrush<_> = GlyphBrushBuilder::using_font(font).build::<wgpu::VertexStepMode, glyph_brush::Extra>();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !matches!(event, WindowEvent::CloseRequested) {
                    return;
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                }

                // Calculate the position to center the text
                let text = "Hello World";
                let scale = 40.0;
                let text_width = text.len() as f32 * scale * 0.5; // Approximate width
                let text_height = scale; // Approximate height
                let screen_position = (
                    (size.width as f32 - text_width) / 2.0,
                    (size.height as f32 - text_height) / 2.0,
                );

                // Queue the text to be drawn
                glyph_brush.queue(Section {
                    screen_position,
                    text: vec![Text::new(text)
                        .with_color([1.0, 0.0, 1.0, 1.0])
                        .with_scale(scale)],
                    ..Section::default()
                });

                // glyph_brush.queue(Section::default().add_text(Text::new("Hello glyph_brush")));
                
                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
} 