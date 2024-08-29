use bytemuck::{bytes_of, cast_slice, cast_slice_mut};
use cube::{Cube, EdgeSticker, Face, RotatedCube};
use cube_3d::{set_up_facelets, update_facelet_colors};
use glam::{vec3, EulerRot, Mat4, Quat, Vec3, Vec4};
use std::{f32::consts::PI, mem::size_of};
use wgpu::{
    include_wgsl, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferDescriptor,
    BufferUsages, Color, CommandEncoderDescriptor, CompareFunction, DepthBiasState,
    DepthStencilState, DeviceDescriptor, Extent3d, Features, FragmentState, IndexFormat, Instance,
    Limits, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PowerPreference,
    PresentMode, PrimitiveState, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, RenderPipelineDescriptor, RequestAdapterOptions, ShaderStages,
    StencilState, StoreOp, SurfaceConfiguration, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureViewDescriptor, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexState, VertexStepMode,
};
use winit::{
    dpi::{LogicalSize, Size},
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    let msaa = 4;

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = Instance::default();

    let surface = unsafe { instance.create_surface(&window).unwrap() };
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let n = 5;

    let num_facelets = cube_3d::num_facelets(n);

    let vertex_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: u64::try_from(num_facelets * cube_3d::VERTICES_PER_FACELET * size_of::<Vec3>())
            .unwrap(),
        usage: BufferUsages::VERTEX,
        mapped_at_creation: true,
    });

    let color_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: u64::try_from(num_facelets * cube_3d::VERTICES_PER_FACELET * size_of::<Vec4>())
            .unwrap(),
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: true,
    });

    let index_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: u64::try_from(num_facelets * cube_3d::INDICES_PER_FACELET * size_of::<u32>())
            .unwrap(),
        usage: BufferUsages::INDEX,
        mapped_at_creation: true,
    });

    let uniform_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: u64::try_from(size_of::<Mat4>()).unwrap(),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(uniform_buffer.as_entire_buffer_binding()),
        }],
    });

    set_up_facelets(
        n,
        cast_slice_mut(&mut *vertex_buffer.slice(..).get_mapped_range_mut()),
        cast_slice_mut(&mut *index_buffer.slice(..).get_mapped_range_mut()),
    );

    let mut cube = Cube::new_random(n, 0);
    let mut cube = RotatedCube::new(&mut cube);
    let mut layers = 2;
    update_facelet_colors(
        &cube.cube,
        cast_slice_mut(&mut *color_buffer.slice(..).get_mapped_range_mut()),
    );

    vertex_buffer.unmap();
    color_buffer.unmap();
    index_buffer.unmap();

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                VertexBufferLayout {
                    array_stride: size_of::<Vec3>().try_into().unwrap(),
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        format: VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
                },
                VertexBufferLayout {
                    array_stride: size_of::<Vec4>().try_into().unwrap(),
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: 0,
                        shader_location: 1,
                    }],
                },
            ],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState {
            count: msaa,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let mut config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::AutoVsync,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let mut msaa_texture = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: msaa,
        dimension: TextureDimension::D2,
        format: swapchain_format,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let mut msaa_view = msaa_texture.create_view(&TextureViewDescriptor::default());

    let mut depth_texture = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: msaa,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let mut depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

    let window = &window;
    event_loop
        .run(move |event, target| {
            let _ = (&instance, &adapter, &shader, &pipeline_layout);

            match event {
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        WindowEvent::Resized(new_size) => {
                            config.width = new_size.width.max(1);
                            config.height = new_size.height.max(1);
                            surface.configure(&device, &config);
                            window.request_redraw();

                            _ = depth_texture;
                            depth_texture = device.create_texture(&TextureDescriptor {
                                label: None,
                                size: Extent3d {
                                    width: config.width,
                                    height: config.height,
                                    depth_or_array_layers: 1,
                                },
                                mip_level_count: 1,
                                sample_count: msaa,
                                dimension: TextureDimension::D2,
                                format: TextureFormat::Depth32Float,
                                usage: TextureUsages::RENDER_ATTACHMENT
                                    | TextureUsages::TEXTURE_BINDING,
                                view_formats: &[],
                            });
                            depth_view =
                                depth_texture.create_view(&TextureViewDescriptor::default());
                            _ = msaa_texture;
                            msaa_texture = device.create_texture(&TextureDescriptor {
                                label: None,
                                size: Extent3d {
                                    width: config.width,
                                    height: config.height,
                                    depth_or_array_layers: 1,
                                },
                                mip_level_count: 1,
                                sample_count: msaa,
                                dimension: TextureDimension::D2,
                                format: swapchain_format,
                                usage: TextureUsages::RENDER_ATTACHMENT
                                    | TextureUsages::TEXTURE_BINDING,
                                view_formats: &[],
                            });
                            msaa_view = msaa_texture.create_view(&TextureViewDescriptor::default());
                        }
                        WindowEvent::RedrawRequested => {
                            let (x, y, z) = cube.orientation.xyz();
                            let quat = Quat::from_euler(
                                EulerRot::ZYX,
                                -f32::from(z) * PI / 2.0,
                                f32::from(y) * PI / 2.0,
                                f32::from(x) * PI / 2.0,
                            );
                            let p = Mat4::perspective_infinite_lh(
                                PI / 4.0,
                                config.width as f32 / config.height as f32,
                                0.01,
                            );
                            let v = Mat4::look_at_lh(vec3(0.0, 5.0, -5.0), Vec3::ZERO, Vec3::Y);
                            let m = Mat4::from_scale(3.0 * Vec3::ONE) * Mat4::from_quat(quat);
                            let mvp = p * v * m;
                            queue.write_buffer(&uniform_buffer, 0, &bytes_of(&mvp));
                            let mut buf =
                                vec![Vec4::ZERO; num_facelets * cube_3d::VERTICES_PER_FACELET];
                            update_facelet_colors(&cube.cube, &mut buf);
                            queue.write_buffer(&color_buffer, 0, &cast_slice(&buf));

                            let frame = surface
                                .get_current_texture()
                                .expect("Failed to acquire next swap chain texture");
                            let view = frame.texture.create_view(&TextureViewDescriptor::default());
                            let mut encoder = device
                                .create_command_encoder(&CommandEncoderDescriptor { label: None });

                            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                                label: None,
                                color_attachments: &[Some(RenderPassColorAttachment {
                                    view: if msaa == 4 { &msaa_view } else { &view },
                                    resolve_target: if msaa == 4 { Some(&view) } else { None },
                                    ops: Operations {
                                        load: LoadOp::Clear(Color::BLACK),
                                        store: StoreOp::Discard,
                                    },
                                })],
                                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                                    view: &depth_view,
                                    depth_ops: Some(Operations {
                                        load: LoadOp::Clear(1.0),
                                        store: StoreOp::Store,
                                    }),
                                    stencil_ops: None,
                                }),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });
                            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
                            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            rpass.set_vertex_buffer(1, color_buffer.slice(..));
                            rpass.set_bind_group(0, &bind_group, &[]);
                            rpass.set_pipeline(&render_pipeline);
                            rpass.draw_indexed(
                                0..(num_facelets * cube_3d::INDICES_PER_FACELET) as u32,
                                0,
                                0..1,
                            );
                            drop(rpass);

                            queue.submit(Some(encoder.finish()));
                            frame.present();
                            window.request_redraw();
                        }
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::KeyboardInput {
                            device_id: _,
                            event,
                            is_synthetic: _,
                        } => {
                            if event.state == ElementState::Pressed && !event.repeat {
                                match event.physical_key {
                                    PhysicalKey::Code(code) => {
                                        process_code_on_cube(code, &mut cube, &mut layers);
                                    }
                                    PhysicalKey::Unidentified(_) => {}
                                }
                            }
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
        })
        .unwrap();
}

fn process_code_on_cube(code: KeyCode, cube: &mut RotatedCube<'_>, layers: &mut u16) {
    let rot = 0..cube.cube.n;
    let slice = 1..cube.cube.n - 1;
    let wide = 0..*layers;
    match code {
        KeyCode::Escape => {
            *cube.cube = Cube::new_solved(cube.cube.n);
            cube.orientation = EdgeSticker::Uf;
        }
        KeyCode::Digit1 => cube.rotate(Face::F, slice, 3),
        KeyCode::Digit2 => cube.rotate(Face::D, slice, 1),
        KeyCode::Digit3 => {}
        KeyCode::Digit4 => {}
        KeyCode::Digit5 => cube.rotate(Face::L, slice, 1),
        KeyCode::Digit6 => cube.rotate(Face::L, slice, 1),
        KeyCode::Digit7 => {}
        KeyCode::Digit8 => {}
        KeyCode::Digit9 => cube.rotate(Face::D, slice, 3),
        KeyCode::Digit0 => cube.rotate(Face::F, slice, 1),

        KeyCode::KeyQ => cube.rotate(Face::F, rot, 3),
        KeyCode::KeyW => cube.rotate(Face::B, 0..1, 1),
        KeyCode::KeyE => cube.rotate(Face::L, 0..1, 3),
        KeyCode::KeyR => cube.rotate(Face::L, wide, 3),
        KeyCode::KeyT => cube.rotate(Face::R, rot, 1),
        KeyCode::KeyY => cube.rotate(Face::R, rot, 1),
        KeyCode::KeyU => cube.rotate(Face::R, wide, 1),
        KeyCode::KeyI => cube.rotate(Face::R, 0..1, 1),
        KeyCode::KeyO => cube.rotate(Face::B, 0..1, 3),
        KeyCode::KeyP => cube.rotate(Face::F, rot, 1),

        KeyCode::KeyA => cube.rotate(Face::U, rot, 3),
        KeyCode::KeyS => cube.rotate(Face::D, 0..1, 1),
        KeyCode::KeyD => cube.rotate(Face::L, 0..1, 1),
        KeyCode::KeyF => cube.rotate(Face::U, 0..1, 3),
        KeyCode::KeyG => cube.rotate(Face::F, 0..1, 3),
        KeyCode::KeyH => cube.rotate(Face::F, 0..1, 1),
        KeyCode::KeyJ => cube.rotate(Face::U, 0..1, 1),
        KeyCode::KeyK => cube.rotate(Face::R, 0..1, 3),
        KeyCode::KeyL => cube.rotate(Face::D, 0..1, 3),
        KeyCode::Semicolon => cube.rotate(Face::U, rot, 1),

        KeyCode::KeyZ => cube.rotate(Face::D, wide, 1),
        KeyCode::KeyX => cube.rotate(Face::L, slice, 3),
        KeyCode::KeyC => cube.rotate(Face::U, wide, 3),
        KeyCode::KeyV => cube.rotate(Face::L, wide, 1),
        KeyCode::KeyB => cube.rotate(Face::R, rot, 3),
        KeyCode::KeyN => cube.rotate(Face::R, rot, 3),
        KeyCode::KeyM => cube.rotate(Face::R, wide, 3),
        KeyCode::Comma => cube.rotate(Face::U, wide, 3),
        KeyCode::Period => cube.rotate(Face::L, slice, 1),
        KeyCode::Slash => cube.rotate(Face::D, wide, 3),
        _ => {}
    }
}

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    }
    let window = builder
        .with_inner_size(Size::Logical(LogicalSize {
            width: 150.0,
            height: 150.0,
        }))
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window));
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
