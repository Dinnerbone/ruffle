use lyon::path::Path;
use lyon::tessellation::{
    self,
    geometry_builder::{BuffersBuilder, FillVertexConstructor, VertexBuffers},
    FillAttributes, FillTessellator, StrokeAttributes, StrokeTessellator, StrokeVertexConstructor,
};
use ruffle_core::backend::render::swf::{self, FillStyle};
use ruffle_core::backend::render::{
    BitmapHandle, BitmapInfo, Color, Letterbox, RenderBackend, ShapeHandle, Transform,
};
use ruffle_core::shape_utils::{DrawCommand, DrawPath};
use std::convert::TryInto;
use swf::{CharacterId, DefineBitsLossless, Glyph, Shape, Twips};

use bytemuck::{Pod, Zeroable};
use futures::executor::block_on;
use std::rc::Rc;
use wgpu::{vertex_attr_array, BindGroupDescriptor, BufferDescriptor, PipelineLayout, TimeOut};
use winit::window::Window;

type Error = Box<dyn std::error::Error>;

pub struct WGPURenderBackend {
    window_surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    color_bind_layout: wgpu::BindGroupLayout,
    color_pipeline: wgpu::RenderPipeline,
    bitmap_bind_layout: wgpu::BindGroupLayout,
    bitmap_pipeline: wgpu::RenderPipeline,
    gradient_bind_layout: wgpu::BindGroupLayout,
    gradient_pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    current_frame: Option<(wgpu::SwapChainOutput, wgpu::CommandEncoder)>,
    meshes: Vec<Mesh>,
    viewport_width: f32,
    viewport_height: f32,
    view_matrix: [[f32; 4]; 4],
    textures: Vec<(swf::CharacterId, Texture)>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Transforms {
    view_matrix: [[f32; 4]; 4],
    world_matrix: [[f32; 4]; 4],
}

unsafe impl Pod for Transforms {}
unsafe impl Zeroable for Transforms {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct TextureTransforms {
    u_matrix: [[f32; 4]; 4],
}

unsafe impl Pod for TextureTransforms {}
unsafe impl Zeroable for TextureTransforms {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ColorAdjustments {
    mult_color: [f32; 4],
    add_color: [f32; 4],
}

unsafe impl Pod for ColorAdjustments {}
unsafe impl Zeroable for ColorAdjustments {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct GPUVertex {
    position: [f32; 2],
    color: [f32; 4],
}

unsafe impl Pod for GPUVertex {}
unsafe impl Zeroable for GPUVertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct GradientUniforms {
    gradient_type: i32,
    num_colors: u32,
    repeat_mode: i32,
    focal_point: f32,
    // TODO: pack this more efficiently. Alignment forces a float[16] to be aligned as a float4[16].
    ratios: [[f32; 4]; 16],
    colors: [[f32; 4]; 16],
}

unsafe impl Pod for GradientUniforms {}
unsafe impl Zeroable for GradientUniforms {}

fn create_pipeline_descriptor<'a>(
    vertex_shader: &'a wgpu::ShaderModule,
    fragment_shader: &'a wgpu::ShaderModule,
    pipeline_layout: &'a PipelineLayout,
    depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
    color_states: &'a [wgpu::ColorStateDescriptor],
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vertex_shader,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fragment_shader,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states,
        depth_stencil_state,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: std::mem::size_of::<GPUVertex>() as u64,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &vertex_attr_array![
                    0 => Float2,
                    1 => Float4
                ],
            }],
        },
    }
}

fn create_color_pipeline(
    device: &wgpu::Device,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
) -> (wgpu::BindGroupLayout, wgpu::RenderPipeline) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
        ],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let pipeline_descriptor = create_pipeline_descriptor(
        vertex_shader,
        fragment_shader,
        &pipeline_layout,
        depth_stencil_state,
        &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            color_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }],
    );

    (
        bind_group_layout,
        device.create_render_pipeline(&pipeline_descriptor),
    )
}

fn create_bitmap_pipeline(
    device: &wgpu::Device,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
) -> (wgpu::BindGroupLayout, wgpu::RenderPipeline) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    component_type: wgpu::TextureComponentType::Float,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
            },
        ],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let pipeline_descriptor = create_pipeline_descriptor(
        vertex_shader,
        fragment_shader,
        &pipeline_layout,
        depth_stencil_state,
        &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            color_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }],
    );

    (
        bind_group_layout,
        device.create_render_pipeline(&pipeline_descriptor),
    )
}

fn create_gradient_pipeline(
    device: &wgpu::Device,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
) -> (wgpu::BindGroupLayout, wgpu::RenderPipeline) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
        ],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let pipeline_descriptor = create_pipeline_descriptor(
        vertex_shader,
        fragment_shader,
        &pipeline_layout,
        depth_stencil_state,
        &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            color_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }],
    );

    (
        bind_group_layout,
        device.create_render_pipeline(&pipeline_descriptor),
    )
}

impl WGPURenderBackend {
    pub fn new(window: Rc<Window>) -> Result<Self, Error> {
        let size = window.inner_size().to_logical(window.scale_factor());
        let window_surface = wgpu::Surface::create(window.as_ref());

        let adapter = block_on(wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            },
            wgpu::BackendBit::PRIMARY,
        ))
        .unwrap();

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        }));

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&window_surface, &swap_chain_desc);

        let color_vs_bytes = include_bytes!("../shaders/color.vert.spv");
        let color_vs = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(
            &color_vs_bytes[..],
        ))?);
        let color_fs_bytes = include_bytes!("../shaders/color.frag.spv");
        let color_fs = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(
            &color_fs_bytes[..],
        ))?);
        let texture_vs_bytes = include_bytes!("../shaders/texture.vert.spv");
        let texture_vs = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(
            &texture_vs_bytes[..],
        ))?);
        let gradient_fs_bytes = include_bytes!("../shaders/gradient.frag.spv");
        let gradient_fs = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(
            &gradient_fs_bytes[..],
        ))?);
        let bitmap_fs_bytes = include_bytes!("../shaders/bitmap.frag.spv");
        let bitmap_fs = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(
            &bitmap_fs_bytes[..],
        ))?);

        let depth_stencil_state = Some(wgpu::DepthStencilStateDescriptor {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Greater,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        });

        let (color_bind_layout, color_pipeline) =
            create_color_pipeline(&device, &color_vs, &color_fs, depth_stencil_state.clone());

        let (bitmap_bind_layout, bitmap_pipeline) = create_bitmap_pipeline(
            &device,
            &texture_vs,
            &bitmap_fs,
            depth_stencil_state.clone(),
        );

        let (gradient_bind_layout, gradient_pipeline) =
            create_gradient_pipeline(&device, &texture_vs, &gradient_fs, depth_stencil_state);

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: swap_chain_desc.width,
                height: swap_chain_desc.height,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });

        let depth_texture_view = depth_texture.create_default_view();

        Ok(Self {
            window_surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            color_bind_layout,
            color_pipeline,
            bitmap_bind_layout,
            bitmap_pipeline,
            gradient_bind_layout,
            gradient_pipeline,
            depth_texture_view,
            current_frame: None,
            meshes: Vec::new(),
            viewport_width: size.width as f32,
            viewport_height: size.height as f32,
            view_matrix: [[0.0; 4]; 4],
            textures: Vec::new(),
        })
    }

    #[allow(clippy::cognitive_complexity)]
    fn register_shape_internal(&mut self, shape: &swf::Shape) -> ShapeHandle {
        let handle = ShapeHandle(self.meshes.len());
        let paths = ruffle_core::shape_utils::swf_shape_to_paths(shape);

        use lyon::tessellation::{FillOptions, StrokeOptions};

        let transforms_ubo = self.device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<Transforms>() as u64,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let colors_ubo = self.device.create_buffer(&BufferDescriptor {
            label: Some("colors_ubo"),
            size: std::mem::size_of::<ColorAdjustments>() as u64,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let mut draws = Vec::new();

        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();
        let mut lyon_mesh: VertexBuffers<_, u16> = VertexBuffers::new();

        #[allow(clippy::too_many_arguments)]
        fn flush_draw(
            draw: IncompleteDrawType,
            draws: &mut Vec<Draw>,
            lyon_mesh: &mut VertexBuffers<GPUVertex, u16>,
            device: &wgpu::Device,
            transforms_ubo: &wgpu::Buffer,
            colors_ubo: &wgpu::Buffer,
            color_bind_layout: &wgpu::BindGroupLayout,
            bitmap_bind_layout: &wgpu::BindGroupLayout,
            gradient_bind_layout: &wgpu::BindGroupLayout,
        ) {
            if lyon_mesh.vertices.is_empty() {
                return;
            }

            let vbo = device.create_buffer_with_data(
                bytemuck::cast_slice(&lyon_mesh.vertices),
                wgpu::BufferUsage::VERTEX,
            );

            let ibo = device.create_buffer_with_data(
                bytemuck::cast_slice(&lyon_mesh.indices),
                wgpu::BufferUsage::INDEX,
            );

            draws.push(draw.build(
                device,
                transforms_ubo,
                colors_ubo,
                vbo,
                ibo,
                lyon_mesh.indices.len() as u32,
                color_bind_layout,
                bitmap_bind_layout,
                gradient_bind_layout,
            ));

            *lyon_mesh = VertexBuffers::new();
        }

        for path in paths {
            match path {
                DrawPath::Fill { style, commands } => match style {
                    FillStyle::Color(color) => {
                        let color = [
                            f32::from(color.r) / 255.0,
                            f32::from(color.g) / 255.0,
                            f32::from(color.b) / 255.0,
                            f32::from(color.a) / 255.0,
                        ];

                        let mut buffers_builder =
                            BuffersBuilder::new(&mut lyon_mesh, RuffleVertexCtor { color });

                        if let Err(e) = fill_tess.tessellate_path(
                            &ruffle_path_to_lyon_path(commands, true),
                            &FillOptions::even_odd(),
                            &mut buffers_builder,
                        ) {
                            // This may just be a degenerate path; skip it.
                            log::error!("Tessellation failure: {:?}", e);
                            continue;
                        }
                    }
                    FillStyle::LinearGradient(gradient) => {
                        flush_draw(
                            IncompleteDrawType::Color,
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );

                        let mut buffers_builder = BuffersBuilder::new(
                            &mut lyon_mesh,
                            RuffleVertexCtor {
                                color: [1.0, 1.0, 1.0, 1.0],
                            },
                        );

                        if let Err(e) = fill_tess.tessellate_path(
                            &ruffle_path_to_lyon_path(commands, true),
                            &FillOptions::even_odd(),
                            &mut buffers_builder,
                        ) {
                            // This may just be a degenerate path; skip it.
                            log::error!("Tessellation failure: {:?}", e);
                            continue;
                        }

                        let mut colors: [[f32; 4]; 16] = Default::default();
                        let mut ratios: [[f32; 4]; 16] = Default::default();
                        for (i, record) in gradient.records.iter().enumerate() {
                            if i >= 16 {
                                // TODO: we need to support these!
                                break;
                            }
                            colors[i] = [
                                f32::from(record.color.r) / 255.0,
                                f32::from(record.color.g) / 255.0,
                                f32::from(record.color.b) / 255.0,
                                f32::from(record.color.a) / 255.0,
                            ];
                            ratios[i] = [f32::from(record.ratio) / 255.0, 0.0, 0.0, 0.0];
                        }

                        let uniforms = GradientUniforms {
                            gradient_type: 0,
                            ratios,
                            colors,
                            num_colors: gradient.records.len() as u32,
                            repeat_mode: 0,
                            focal_point: 0.0,
                        };
                        let matrix = swf_to_gl_matrix(gradient.matrix.clone());

                        flush_draw(
                            IncompleteDrawType::Gradient {
                                texture_transform: matrix,
                                gradient: uniforms,
                            },
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );
                    }
                    FillStyle::RadialGradient(gradient) => {
                        flush_draw(
                            IncompleteDrawType::Color,
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );

                        let mut buffers_builder = BuffersBuilder::new(
                            &mut lyon_mesh,
                            RuffleVertexCtor {
                                color: [1.0, 1.0, 1.0, 1.0],
                            },
                        );

                        if let Err(e) = fill_tess.tessellate_path(
                            &ruffle_path_to_lyon_path(commands, true),
                            &FillOptions::even_odd(),
                            &mut buffers_builder,
                        ) {
                            // This may just be a degenerate path; skip it.
                            log::error!("Tessellation failure: {:?}", e);
                            continue;
                        }

                        let mut colors: [[f32; 4]; 16] = Default::default();
                        let mut ratios: [[f32; 4]; 16] = Default::default();
                        for (i, record) in gradient.records.iter().enumerate() {
                            if i >= 16 {
                                // TODO: we need to support these!
                                break;
                            }
                            colors[i] = [
                                f32::from(record.color.r) / 255.0,
                                f32::from(record.color.g) / 255.0,
                                f32::from(record.color.b) / 255.0,
                                f32::from(record.color.a) / 255.0,
                            ];
                            ratios[i] = [f32::from(record.ratio) / 255.0, 0.0, 0.0, 0.0];
                        }

                        let uniforms = GradientUniforms {
                            gradient_type: 1,
                            ratios,
                            colors,
                            num_colors: gradient.records.len() as u32,
                            repeat_mode: 0,
                            focal_point: 0.0,
                        };
                        let matrix = swf_to_gl_matrix(gradient.matrix.clone());

                        flush_draw(
                            IncompleteDrawType::Gradient {
                                texture_transform: matrix,
                                gradient: uniforms,
                            },
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );
                    }
                    FillStyle::FocalGradient {
                        gradient,
                        focal_point,
                    } => {
                        flush_draw(
                            IncompleteDrawType::Color,
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );

                        let mut buffers_builder = BuffersBuilder::new(
                            &mut lyon_mesh,
                            RuffleVertexCtor {
                                color: [1.0, 1.0, 1.0, 1.0],
                            },
                        );

                        if let Err(e) = fill_tess.tessellate_path(
                            &ruffle_path_to_lyon_path(commands, true),
                            &FillOptions::even_odd(),
                            &mut buffers_builder,
                        ) {
                            // This may just be a degenerate path; skip it.
                            log::error!("Tessellation failure: {:?}", e);
                            continue;
                        }

                        let mut colors: [[f32; 4]; 16] = Default::default();
                        let mut ratios: [[f32; 4]; 16] = Default::default();
                        for (i, record) in gradient.records.iter().enumerate() {
                            if i >= 16 {
                                // TODO: we need to support these!
                                break;
                            }
                            colors[i] = [
                                f32::from(record.color.r) / 255.0,
                                f32::from(record.color.g) / 255.0,
                                f32::from(record.color.b) / 255.0,
                                f32::from(record.color.a) / 255.0,
                            ];
                            ratios[i] = [f32::from(record.ratio) / 255.0, 0.0, 0.0, 0.0];
                        }

                        let uniforms = GradientUniforms {
                            gradient_type: 1,
                            ratios,
                            colors,
                            num_colors: gradient.records.len() as u32,
                            repeat_mode: 0,
                            focal_point: *focal_point,
                        };
                        let matrix = swf_to_gl_matrix(gradient.matrix.clone());

                        flush_draw(
                            IncompleteDrawType::Gradient {
                                texture_transform: matrix,
                                gradient: uniforms,
                            },
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );
                    }
                    FillStyle::Bitmap {
                        id,
                        matrix,
                        is_smoothed,
                        is_repeating,
                    } => {
                        flush_draw(
                            IncompleteDrawType::Color,
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );

                        let mut buffers_builder = BuffersBuilder::new(
                            &mut lyon_mesh,
                            RuffleVertexCtor {
                                color: [1.0, 1.0, 1.0, 1.0],
                            },
                        );

                        if let Err(e) = fill_tess.tessellate_path(
                            &ruffle_path_to_lyon_path(commands, true),
                            &FillOptions::even_odd(),
                            &mut buffers_builder,
                        ) {
                            // This may just be a degenerate path; skip it.
                            log::error!("Tessellation failure: {:?}", e);
                            continue;
                        }

                        let texture = &self
                            .textures
                            .iter()
                            .find(|(other_id, _tex)| *other_id == *id)
                            .unwrap()
                            .1;
                        let texture_view = texture.texture.create_default_view();

                        flush_draw(
                            IncompleteDrawType::Bitmap {
                                texture_transform: swf_bitmap_to_gl_matrix(
                                    matrix.clone(),
                                    texture.width,
                                    texture.height,
                                ),
                                is_smoothed: *is_smoothed,
                                is_repeating: *is_repeating,
                                texture_view,
                                id: *id,
                            },
                            &mut draws,
                            &mut lyon_mesh,
                            &self.device,
                            &transforms_ubo,
                            &colors_ubo,
                            &self.color_bind_layout,
                            &self.bitmap_bind_layout,
                            &self.gradient_bind_layout,
                        );
                    }
                },
                DrawPath::Stroke {
                    style,
                    commands,
                    is_closed,
                } => {
                    let color = [
                        f32::from(style.color.r) / 255.0,
                        f32::from(style.color.g) / 255.0,
                        f32::from(style.color.b) / 255.0,
                        f32::from(style.color.a) / 255.0,
                    ];

                    let mut buffers_builder =
                        BuffersBuilder::new(&mut lyon_mesh, RuffleVertexCtor { color });

                    // TODO(Herschel): 0 width indicates "hairline".
                    let width = if style.width.to_pixels() >= 1.0 {
                        style.width.to_pixels() as f32
                    } else {
                        1.0
                    };

                    let mut options = StrokeOptions::default()
                        .with_line_width(width)
                        .with_line_join(match style.join_style {
                            swf::LineJoinStyle::Round => tessellation::LineJoin::Round,
                            swf::LineJoinStyle::Bevel => tessellation::LineJoin::Bevel,
                            swf::LineJoinStyle::Miter(_) => tessellation::LineJoin::MiterClip,
                        })
                        .with_start_cap(match style.start_cap {
                            swf::LineCapStyle::None => tessellation::LineCap::Butt,
                            swf::LineCapStyle::Round => tessellation::LineCap::Round,
                            swf::LineCapStyle::Square => tessellation::LineCap::Square,
                        })
                        .with_end_cap(match style.end_cap {
                            swf::LineCapStyle::None => tessellation::LineCap::Butt,
                            swf::LineCapStyle::Round => tessellation::LineCap::Round,
                            swf::LineCapStyle::Square => tessellation::LineCap::Square,
                        });

                    if let swf::LineJoinStyle::Miter(limit) = style.join_style {
                        options = options.with_miter_limit(limit);
                    }

                    if let Err(e) = stroke_tess.tessellate_path(
                        &ruffle_path_to_lyon_path(commands, is_closed),
                        &options,
                        &mut buffers_builder,
                    ) {
                        // This may just be a degenerate path; skip it.
                        log::error!("Tessellation failure: {:?}", e);
                        continue;
                    }
                }
            }
        }

        flush_draw(
            IncompleteDrawType::Color,
            &mut draws,
            &mut lyon_mesh,
            &self.device,
            &transforms_ubo,
            &colors_ubo,
            &self.color_bind_layout,
            &self.bitmap_bind_layout,
            &self.gradient_bind_layout,
        );

        self.meshes.push(Mesh {
            draws,
            transforms: transforms_ubo,
            colors: colors_ubo,
        });

        handle
    }

    fn build_matrices(&mut self) {
        self.view_matrix = [
            [1.0 / (self.viewport_width as f32 / 2.0), 0.0, 0.0, 0.0],
            [0.0, -1.0 / (self.viewport_height as f32 / 2.0), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0, 1.0],
        ];
    }
}

impl RenderBackend for WGPURenderBackend {
    fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        self.swap_chain_desc.width = width;
        self.swap_chain_desc.height = height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.window_surface, &self.swap_chain_desc);

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });

        self.depth_texture_view = depth_texture.create_default_view();
        self.viewport_width = width as f32;
        self.viewport_height = height as f32;
        self.build_matrices();
    }

    fn register_shape(&mut self, shape: &Shape) -> ShapeHandle {
        self.register_shape_internal(shape)
    }

    fn register_glyph_shape(&mut self, glyph: &Glyph) -> ShapeHandle {
        let shape = swf::Shape {
            version: 2,
            id: 0,
            shape_bounds: Default::default(),
            edge_bounds: Default::default(),
            has_fill_winding_rule: false,
            has_non_scaling_strokes: false,
            has_scaling_strokes: true,
            styles: swf::ShapeStyles {
                fill_styles: vec![FillStyle::Color(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                })],
                line_styles: vec![],
            },
            shape: glyph.shape_records.clone(),
        };
        self.register_shape_internal(&shape)
    }

    fn register_bitmap_jpeg(
        &mut self,
        id: u16,
        data: &[u8],
        jpeg_tables: Option<&[u8]>,
    ) -> BitmapInfo {
        let data = ruffle_core::backend::render::glue_tables_to_jpeg(data, jpeg_tables);
        self.register_bitmap_jpeg_2(id, &data[..])
    }

    fn register_bitmap_jpeg_2(&mut self, id: u16, data: &[u8]) -> BitmapInfo {
        let data = ruffle_core::backend::render::remove_invalid_jpeg_data(data);

        let mut decoder = jpeg_decoder::Decoder::new(&data[..]);
        decoder.read_info().unwrap();
        let metadata = decoder.info().unwrap();
        let decoded_data = decoder.decode().expect("failed to decode image");
        let extent = wgpu::Extent3d {
            width: metadata.width as u32,
            height: metadata.height as u32,
            depth: 1,
        };
        let mut as_rgba = Vec::with_capacity((extent.width * extent.height * 4) as usize);
        for i in (0..decoded_data.len()).step_by(3) {
            as_rgba.push(decoded_data[i]);
            as_rgba.push(decoded_data[i + 1]);
            as_rgba.push(decoded_data[i + 2]);
            as_rgba.push(255);
        }

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("JPEG2 image"),
            size: extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let buffer = self
            .device
            .create_buffer_with_data(&as_rgba[..], wgpu::BufferUsage::COPY_SRC);
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("JPEG2 image encoder"),
            });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * extent.width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            extent,
        );
        self.queue.submit(&[encoder.finish()]);

        let handle = BitmapHandle(self.textures.len());
        self.textures.push((
            id,
            Texture {
                texture,
                width: metadata.width.into(),
                height: metadata.height.into(),
            },
        ));

        BitmapInfo {
            handle,
            width: metadata.width,
            height: metadata.height,
        }
    }

    fn register_bitmap_jpeg_3(
        &mut self,
        id: u16,
        jpeg_data: &[u8],
        alpha_data: &[u8],
    ) -> BitmapInfo {
        let (width, height, rgba) =
            ruffle_core::backend::render::define_bits_jpeg_to_rgba(jpeg_data, alpha_data)
                .expect("Error decoding DefineBitsJPEG3");
        let extent = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let buffer = self
            .device
            .create_buffer_with_data(&rgba[..], wgpu::BufferUsage::COPY_SRC);
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * extent.width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            extent,
        );
        self.queue.submit(&[encoder.finish()]);

        let handle = BitmapHandle(self.textures.len());
        self.textures.push((
            id,
            Texture {
                texture,
                width,
                height,
            },
        ));

        BitmapInfo {
            handle,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        }
    }

    fn register_bitmap_png(&mut self, swf_tag: &DefineBitsLossless) -> BitmapInfo {
        let decoded_data = ruffle_core::backend::render::define_bits_lossless_to_rgba(swf_tag)
            .expect("Error decoding DefineBitsLossless");
        let extent = wgpu::Extent3d {
            width: swf_tag.width as u32,
            height: swf_tag.height as u32,
            depth: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let buffer = self
            .device
            .create_buffer_with_data(&decoded_data[..], wgpu::BufferUsage::COPY_SRC);
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * extent.width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            extent,
        );
        self.queue.submit(&[encoder.finish()]);

        let handle = BitmapHandle(self.textures.len());
        self.textures.push((
            swf_tag.id,
            Texture {
                texture,
                width: swf_tag.width.into(),
                height: swf_tag.height.into(),
            },
        ));

        BitmapInfo {
            handle,
            width: swf_tag.width,
            height: swf_tag.height,
        }
    }

    fn begin_frame(&mut self) {
        assert!(self.current_frame.is_none());
        self.current_frame = match self.swap_chain.get_next_texture() {
            Ok(frame) => Some((
                frame,
                self.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
            )),
            Err(TimeOut) => {
                log::warn!("Couldn't begin new render frame: timed out whilst aquiring new swapchain output");
                None
            }
        };
    }

    fn clear(&mut self, color: Color) {
        if let Some((swap_chain_output, encoder)) = &mut self.current_frame {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &swap_chain_output.view,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: f64::from(color.r) / 255.0,
                        g: f64::from(color.g) / 255.0,
                        b: f64::from(color.b) / 255.0,
                        a: f64::from(color.a) / 255.0,
                    },
                    resolve_target: None,
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture_view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_depth: 0.0,
                    clear_stencil: 0,
                }),
            });
        }
    }

    fn render_bitmap(&mut self, _bitmap: BitmapHandle, _transform: &Transform) {}

    fn render_shape(&mut self, shape: ShapeHandle, transform: &Transform) {
        let (swap_chain_output, encoder) =
            if let Some((swap_chain_output, encoder)) = &mut self.current_frame {
                (swap_chain_output, encoder)
            } else {
                return;
            };

        let mesh = &self.meshes[shape.0];

        let world_matrix = [
            [transform.matrix.a, transform.matrix.b, 0.0, 0.0],
            [transform.matrix.c, transform.matrix.d, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [
                transform.matrix.tx.to_pixels() as f32,
                transform.matrix.ty.to_pixels() as f32,
                0.0,
                1.0,
            ],
        ];

        let mult_color = [
            transform.color_transform.r_mult,
            transform.color_transform.g_mult,
            transform.color_transform.b_mult,
            transform.color_transform.a_mult,
        ];

        let add_color = [
            transform.color_transform.r_add,
            transform.color_transform.g_add,
            transform.color_transform.b_add,
            transform.color_transform.a_add,
        ];

        let transforms_temp = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&[Transforms {
                view_matrix: self.view_matrix,
                world_matrix,
            }]),
            wgpu::BufferUsage::COPY_SRC,
        );

        let colors_temp = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&[ColorAdjustments {
                mult_color,
                add_color,
            }]),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &transforms_temp,
            0,
            &mesh.transforms,
            0,
            std::mem::size_of::<Transforms>() as u64,
        );
        encoder.copy_buffer_to_buffer(
            &colors_temp,
            0,
            &mesh.colors,
            0,
            std::mem::size_of::<ColorAdjustments>() as u64,
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &swap_chain_output.view,
                load_op: wgpu::LoadOp::Load,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color::WHITE,
                resolve_target: None,
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &self.depth_texture_view,
                depth_load_op: wgpu::LoadOp::Load,
                depth_store_op: wgpu::StoreOp::Store,
                stencil_load_op: wgpu::LoadOp::Load,
                stencil_store_op: wgpu::StoreOp::Store,
                clear_depth: 0.0,
                clear_stencil: 0,
            }),
        });

        for draw in &mesh.draws {
            match &draw.draw_type {
                DrawType::Color => {
                    render_pass.set_pipeline(&self.color_pipeline);
                }
                DrawType::Gradient { .. } => {
                    render_pass.set_pipeline(&self.gradient_pipeline);
                }
                DrawType::Bitmap { .. } => {
                    render_pass.set_pipeline(&self.bitmap_pipeline);
                }
            }

            render_pass.set_bind_group(0, &draw.bind_group, &[]);
            render_pass.set_vertex_buffer(0, &draw.vertex_buffer, 0, 0);
            render_pass.set_index_buffer(&draw.index_buffer, 0, 0);

            render_pass.draw_indexed(0..draw.index_count, 0, 0..1);
        }
    }

    fn end_frame(&mut self) {
        if let Some((_frame, encoder)) = self.current_frame.take() {
            self.queue.submit(&[encoder.finish()]);
        }
    }

    fn draw_letterbox(&mut self, _letterbox: Letterbox) {}

    fn push_mask(&mut self) {}

    fn activate_mask(&mut self) {}

    fn pop_mask(&mut self) {}
}

fn point(x: Twips, y: Twips) -> lyon::math::Point {
    lyon::math::Point::new(x.to_pixels() as f32, y.to_pixels() as f32)
}

fn ruffle_path_to_lyon_path(commands: Vec<DrawCommand>, is_closed: bool) -> Path {
    let mut builder = Path::builder();
    for cmd in commands {
        match cmd {
            DrawCommand::MoveTo { x, y } => {
                builder.move_to(point(x, y));
            }
            DrawCommand::LineTo { x, y } => {
                builder.line_to(point(x, y));
            }
            DrawCommand::CurveTo { x1, y1, x2, y2 } => {
                builder.quadratic_bezier_to(point(x1, y1), point(x2, y2));
            }
        }
    }

    if is_closed {
        builder.close();
    }

    builder.build()
}

#[derive(Debug)]
struct Texture {
    width: u32,
    height: u32,
    texture: wgpu::Texture,
}

#[derive(Debug)]
struct Mesh {
    draws: Vec<Draw>,
    transforms: wgpu::Buffer,
    colors: wgpu::Buffer,
}

#[derive(Debug)]
struct Draw {
    draw_type: DrawType,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    index_count: u32,
}

#[derive(Debug)]
enum DrawType {
    Color,
    Gradient {
        texture_transforms: wgpu::Buffer,
        gradient: wgpu::Buffer,
    },
    Bitmap {
        texture_transforms: wgpu::Buffer,
        texture_view: wgpu::TextureView,
        id: CharacterId,
    },
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum IncompleteDrawType {
    Color,
    Gradient {
        texture_transform: [[f32; 4]; 4],
        gradient: GradientUniforms,
    },
    Bitmap {
        texture_transform: [[f32; 4]; 4],
        is_smoothed: bool,
        is_repeating: bool,
        texture_view: wgpu::TextureView,
        id: CharacterId,
    },
}

impl IncompleteDrawType {
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        self,
        device: &wgpu::Device,
        transforms_ubo: &wgpu::Buffer,
        colors_ubo: &wgpu::Buffer,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        index_count: u32,
        color_bind_layout: &wgpu::BindGroupLayout,
        bitmap_bind_layout: &wgpu::BindGroupLayout,
        gradient_bind_layout: &wgpu::BindGroupLayout,
    ) -> Draw {
        match self {
            IncompleteDrawType::Color => {
                let bind_group = device.create_bind_group(&BindGroupDescriptor {
                    layout: color_bind_layout,
                    bindings: &[
                        wgpu::Binding {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: transforms_ubo,
                                range: 0..std::mem::size_of::<Transforms>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 1,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: colors_ubo,
                                range: 0..std::mem::size_of::<ColorAdjustments>() as u64,
                            },
                        },
                    ],
                    label: None,
                });

                Draw {
                    draw_type: DrawType::Color,
                    vertex_buffer,
                    index_buffer,
                    bind_group,
                    index_count,
                }
            }
            IncompleteDrawType::Gradient {
                texture_transform,
                gradient,
            } => {
                let tex_transforms_ubo = device.create_buffer_with_data(
                    bytemuck::cast_slice(&[texture_transform]),
                    wgpu::BufferUsage::UNIFORM,
                );

                let gradient_ubo = device.create_buffer_with_data(
                    bytemuck::cast_slice(&[gradient]),
                    wgpu::BufferUsage::UNIFORM,
                );

                let bind_group = device.create_bind_group(&BindGroupDescriptor {
                    layout: gradient_bind_layout,
                    bindings: &[
                        wgpu::Binding {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: transforms_ubo,
                                range: 0..std::mem::size_of::<Transforms>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 1,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &tex_transforms_ubo,
                                range: 0..std::mem::size_of::<TextureTransforms>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 2,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: colors_ubo,
                                range: 0..std::mem::size_of::<ColorAdjustments>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 3,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &gradient_ubo,
                                range: 0..std::mem::size_of::<GradientUniforms>() as u64,
                            },
                        },
                    ],
                    label: None,
                });

                Draw {
                    draw_type: DrawType::Gradient {
                        texture_transforms: tex_transforms_ubo,
                        gradient: gradient_ubo,
                    },
                    vertex_buffer,
                    index_buffer,
                    bind_group,
                    index_count,
                }
            }
            IncompleteDrawType::Bitmap {
                texture_transform,
                is_smoothed,
                is_repeating,
                texture_view,
                id,
            } => {
                let tex_transforms_ubo = device.create_buffer_with_data(
                    bytemuck::cast_slice(&[texture_transform]),
                    wgpu::BufferUsage::UNIFORM,
                );

                let address_mode = if is_repeating {
                    wgpu::AddressMode::Repeat
                } else {
                    wgpu::AddressMode::ClampToEdge
                };

                let filter = if is_smoothed {
                    wgpu::FilterMode::Linear
                } else {
                    wgpu::FilterMode::Nearest
                };

                let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: address_mode,
                    address_mode_v: address_mode,
                    address_mode_w: address_mode,
                    mag_filter: filter,
                    min_filter: filter,
                    mipmap_filter: filter,
                    lod_min_clamp: 0.0,
                    lod_max_clamp: 100.0,
                    compare: wgpu::CompareFunction::Undefined,
                });

                let bind_group = device.create_bind_group(&BindGroupDescriptor {
                    layout: bitmap_bind_layout,
                    bindings: &[
                        wgpu::Binding {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: transforms_ubo,
                                range: 0..std::mem::size_of::<Transforms>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 1,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &tex_transforms_ubo,
                                range: 0..std::mem::size_of::<TextureTransforms>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 2,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: colors_ubo,
                                range: 0..std::mem::size_of::<ColorAdjustments>() as u64,
                            },
                        },
                        wgpu::Binding {
                            binding: 3,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::Binding {
                            binding: 4,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                    label: None,
                });

                Draw {
                    draw_type: DrawType::Bitmap {
                        texture_transforms: tex_transforms_ubo,
                        texture_view,
                        id,
                    },
                    vertex_buffer,
                    index_buffer,
                    bind_group,
                    index_count,
                }
            }
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn swf_to_gl_matrix(m: swf::Matrix) -> [[f32; 4]; 4] {
    let tx = m.translate_x.get() as f32;
    let ty = m.translate_y.get() as f32;
    let det = m.scale_x * m.scale_y - m.rotate_skew_1 * m.rotate_skew_0;
    let mut a = m.scale_y / det;
    let mut b = -m.rotate_skew_1 / det;
    let mut c = -(tx * m.scale_y - m.rotate_skew_1 * ty) / det;
    let mut d = -m.rotate_skew_0 / det;
    let mut e = m.scale_x / det;
    let mut f = (tx * m.rotate_skew_0 - m.scale_x * ty) / det;

    a *= 20.0 / 32768.0;
    b *= 20.0 / 32768.0;
    d *= 20.0 / 32768.0;
    e *= 20.0 / 32768.0;

    c /= 32768.0;
    f /= 32768.0;
    c += 0.5;
    f += 0.5;
    [
        [a, d, 0.0, 0.0],
        [b, e, 0., 0.0],
        [c, f, 1.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
}

#[allow(clippy::many_single_char_names)]
fn swf_bitmap_to_gl_matrix(m: swf::Matrix, bitmap_width: u32, bitmap_height: u32) -> [[f32; 4]; 4] {
    let bitmap_width = bitmap_width as f32;
    let bitmap_height = bitmap_height as f32;

    let tx = m.translate_x.get() as f32;
    let ty = m.translate_y.get() as f32;
    let det = m.scale_x * m.scale_y - m.rotate_skew_1 * m.rotate_skew_0;
    let mut a = m.scale_y / det;
    let mut b = -m.rotate_skew_1 / det;
    let mut c = -(tx * m.scale_y - m.rotate_skew_1 * ty) / det;
    let mut d = -m.rotate_skew_0 / det;
    let mut e = m.scale_x / det;
    let mut f = (tx * m.rotate_skew_0 - m.scale_x * ty) / det;

    a *= 20.0 / bitmap_width;
    b *= 20.0 / bitmap_width;
    d *= 20.0 / bitmap_height;
    e *= 20.0 / bitmap_height;

    c /= bitmap_width;
    f /= bitmap_height;

    [
        [a, d, 0.0, 0.0],
        [b, e, 0.0, 0.0],
        [c, f, 1.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
}

struct RuffleVertexCtor {
    color: [f32; 4],
}

impl FillVertexConstructor<GPUVertex> for RuffleVertexCtor {
    fn new_vertex(&mut self, position: lyon::math::Point, _: FillAttributes) -> GPUVertex {
        GPUVertex {
            position: [position.x, position.y],
            color: self.color,
        }
    }
}

impl StrokeVertexConstructor<GPUVertex> for RuffleVertexCtor {
    fn new_vertex(&mut self, position: lyon::math::Point, _: StrokeAttributes) -> GPUVertex {
        GPUVertex {
            position: [position.x, position.y],
            color: self.color,
        }
    }
}
