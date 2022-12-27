use crate::context3d::WgpuContext3D;
use crate::mesh::{Draw, Mesh};
use crate::surface::Surface;
use crate::target::RenderTargetFrame;
use crate::target::TextureTarget;
use crate::uniform_buffer::BufferStorage;
use crate::{
    as_texture, format_list, get_backend_names, BufferDimensions, Descriptors, Error, Globals,
    RenderTarget, SwapChainTarget, Texture, TextureOffscreen, Transforms,
};
use gc_arena::MutationContext;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use ruffle_render::backend::{Context3D, Context3DCommand};
use ruffle_render::backend::{RenderBackend, ShapeHandle, ViewportDimensions};
use ruffle_render::bitmap::{Bitmap, BitmapHandle, BitmapSource};
use ruffle_render::commands::CommandList;
use ruffle_render::error::Error as BitmapError;
use ruffle_render::shape_utils::DistilledShape;
use ruffle_render::tessellator::ShapeTessellator;
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::Arc;
use swf::Color;
use wgpu::Extent3d;

const DEFAULT_SAMPLE_COUNT: u32 = 4;

pub struct WgpuRenderBackend<T: RenderTarget> {
    descriptors: Arc<Descriptors>,
    globals: Globals,
    uniform_buffers_storage: BufferStorage<Transforms>,
    target: T,
    surface: Surface,
    meshes: Vec<Mesh>,
    shape_tessellator: ShapeTessellator,
    // This is currently unused - we just store it to report in
    // `get_viewport_dimensions`
    viewport_scale_factor: f64,
}

impl WgpuRenderBackend<SwapChainTarget> {
    #[cfg(target_family = "wasm")]
    pub async fn for_canvas(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, Error> {
        let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL);
        let surface = instance.create_surface_from_canvas(canvas);
        let descriptors = Self::build_descriptors(
            wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            instance,
            Some(&surface),
            wgpu::PowerPreference::HighPerformance,
            None,
        )
        .await?;
        let target =
            SwapChainTarget::new(surface, &descriptors.adapter, (1, 1), &descriptors.device);
        Self::new(Arc::new(descriptors), target)
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn for_window<W: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &W,
        size: (u32, u32),
        backend: wgpu::Backends,
        power_preference: wgpu::PowerPreference,
        trace_path: Option<&Path>,
    ) -> Result<Self, Error> {
        if wgpu::Backends::SECONDARY.contains(backend) {
            log::warn!(
                "{} graphics backend support may not be fully supported.",
                format_list(&get_backend_names(backend), "and")
            );
        }
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };
        let descriptors = futures::executor::block_on(Self::build_descriptors(
            backend,
            instance,
            Some(&surface),
            power_preference,
            trace_path,
        ))?;
        let target = SwapChainTarget::new(surface, &descriptors.adapter, size, &descriptors.device);
        Self::new(Arc::new(descriptors), target)
    }
}

#[cfg(not(target_family = "wasm"))]
impl WgpuRenderBackend<crate::target::TextureTarget> {
    pub fn for_offscreen(
        size: (u32, u32),
        backend: wgpu::Backends,
        power_preference: wgpu::PowerPreference,
        trace_path: Option<&Path>,
    ) -> Result<Self, Error> {
        if wgpu::Backends::SECONDARY.contains(backend) {
            log::warn!(
                "{} graphics backend support may not be fully supported.",
                format_list(&get_backend_names(backend), "and")
            );
        }
        let instance = wgpu::Instance::new(backend);
        let descriptors = futures::executor::block_on(Self::build_descriptors(
            backend,
            instance,
            None,
            power_preference,
            trace_path,
        ))?;
        let target = crate::target::TextureTarget::new(&descriptors.device, size)?;
        Self::new(Arc::new(descriptors), target)
    }

    pub fn capture_frame(&self, premultiplied_alpha: bool) -> Option<image::RgbaImage> {
        self.target
            .capture(&self.descriptors.device, premultiplied_alpha)
    }
}

impl<T: RenderTarget> WgpuRenderBackend<T> {
    pub fn new(descriptors: Arc<Descriptors>, target: T) -> Result<Self, Error> {
        if target.width() > descriptors.limits.max_texture_dimension_2d
            || target.height() > descriptors.limits.max_texture_dimension_2d
        {
            return Err(format!(
                "Render target texture cannot be larger than {}px on either dimension (requested {} x {})",
                descriptors.limits.max_texture_dimension_2d,
                target.width(),
                target.height()
            )
                .into());
        }

        // TODO: Allow the sample count to be set from command line/settings file.
        let surface = Surface::new(
            &descriptors,
            DEFAULT_SAMPLE_COUNT,
            target.width(),
            target.height(),
            target.format(),
        );

        let mut globals = Globals::new(&descriptors.device, &descriptors.bind_layouts.globals);
        globals.set_resolution(target.width(), target.height());

        let uniform_buffers_storage =
            BufferStorage::from_alignment(descriptors.limits.min_uniform_buffer_offset_alignment);

        Ok(Self {
            descriptors,
            globals,
            uniform_buffers_storage,
            target,
            surface,
            meshes: Vec::new(),
            shape_tessellator: ShapeTessellator::new(),
            viewport_scale_factor: 1.0,
        })
    }

    pub async fn build_descriptors(
        backend: wgpu::Backends,
        instance: wgpu::Instance,
        surface: Option<&wgpu::Surface>,
        power_preference: wgpu::PowerPreference,
        trace_path: Option<&Path>,
    ) -> Result<Descriptors, Error> {
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference,
            compatible_surface: surface,
            force_fallback_adapter: false,
        }).await
            .ok_or_else(|| {
                let names = get_backend_names(backend);
                if names.is_empty() {
                    "Ruffle requires hardware acceleration, but no compatible graphics device was found (no backend provided?)".to_string()
                } else if cfg!(any(windows, target_os = "macos")) {
                    format!("Ruffle does not support OpenGL on {}.", if cfg!(windows) { "Windows" } else { "macOS" })
                } else {
                    format!("Ruffle requires hardware acceleration, but no compatible graphics device was found supporting {}", format_list(&names, "or"))
                }
            })?;

        let (limits, features) = required_limits(&adapter);
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                },
                trace_path,
            )
            .await?;

        Ok(Descriptors::new(adapter, device, queue))
    }

    /// # Safety
    ///
    /// - `raw_device` must be created using `family_index`, `enabled_extensions` and `physical_device_features()`
    /// - `enabled_extensions` must be a superset of `Adapter::required_device_extensions()`
    #[cfg(all(feature = "with_hal", feature = "vulkan"))]
    #[allow(clippy::too_many_arguments)]
    pub async unsafe fn build_descriptors_for_vulkan(
        phd: ash::vk::PhysicalDevice,
        raw_device: ash::Device,
        handle_is_owned: bool,
        enabled_extensions: &[&'static std::ffi::CStr],
        features: wgpu::Features,
        uab_types: wgpu_hal::UpdateAfterBindTypes,
        family_index: u32,
        queue_index: u32,
        trace_path: Option<&Path>,
    ) -> Result<Descriptors, Error> {
        use wgpu_hal::api::Vulkan;
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let instance_hal = instance
            .as_hal::<Vulkan>()
            .expect("Backend made for Vulkan should exist for Vulkan");
        let adapter_hal = instance_hal
            .expose_adapter(phd)
            .expect("expose_adapter should be infallible");
        let open_device = adapter_hal.adapter.device_from_raw(
            raw_device,
            handle_is_owned,
            enabled_extensions,
            features,
            uab_types,
            family_index,
            queue_index,
        )?;
        let adapter = instance.create_adapter_from_hal(adapter_hal);
        let (limits, features) = required_limits(&adapter);
        let (device, queue) = adapter.create_device_from_hal(
            open_device,
            &wgpu::DeviceDescriptor {
                label: None,
                features,
                limits,
            },
            trace_path,
        )?;

        Ok(Descriptors::new(adapter, device, queue))
    }

    /// # Safety
    ///
    /// - The underlying OpenGL ES context must be current.
    /// - The underlying OpenGL ES context must be current when interfacing with any objects returned by
    ///   wgpu-hal.
    #[cfg(all(feature = "with_hal", feature = "gl"))]
    #[allow(clippy::too_many_arguments)]
    pub async unsafe fn build_descriptors_for_gl(
        fun: impl FnMut(&str) -> *const core::ffi::c_void,
        trace_path: Option<&Path>,
    ) -> Result<Descriptors, Error> {
        use wgpu_hal::api::Gles;
        let instance = wgpu::Instance::new(wgpu::Backends::GL);
        let instance_hal = instance
            .as_hal::<Gles>()
            .expect("Backend made for GL should exist for GL");
        let adapter_hal = instance_hal
            .new_external(fun)
            .expect("expose_adapter should be infallible");
        let adapter = instance.create_adapter_from_hal(adapter_hal);
        let (limits, features) = required_limits(&adapter);
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                },
                trace_path,
            )
            .await?;

        Ok(Descriptors::new(adapter, device, queue))
    }

    fn register_shape_internal(
        &mut self,
        shape: DistilledShape,
        bitmap_source: &dyn BitmapSource,
    ) -> Mesh {
        let shape_id = shape.id;
        let lyon_mesh = self
            .shape_tessellator
            .tessellate_shape(shape, bitmap_source);

        let mut draws = Vec::with_capacity(lyon_mesh.len());
        for draw in lyon_mesh {
            let draw_id = draws.len();
            draws.push(Draw::new(self, bitmap_source, draw, shape_id, draw_id));
        }

        Mesh { draws }
    }

    pub fn descriptors(&self) -> &Arc<Descriptors> {
        &self.descriptors
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.descriptors.device
    }
}

impl<T: RenderTarget + 'static> RenderBackend for WgpuRenderBackend<T> {
    fn set_viewport_dimensions(&mut self, dimensions: ViewportDimensions) {
        // Avoid panics from creating 0-sized framebuffers.
        // TODO: find a way to bubble an error when the size is too large
        let width = std::cmp::max(
            std::cmp::min(
                dimensions.width,
                self.descriptors.limits.max_texture_dimension_2d,
            ),
            1,
        );
        let height = std::cmp::max(
            std::cmp::min(
                dimensions.height,
                self.descriptors.limits.max_texture_dimension_2d,
            ),
            1,
        );
        self.target.resize(&self.descriptors.device, width, height);
        self.surface = Surface::new(&self.descriptors, 4, width, height, self.target.format());

        self.globals.set_resolution(width, height);
        self.viewport_scale_factor = dimensions.scale_factor;
    }

    fn create_context3d(
        &mut self,
    ) -> Result<Box<dyn ruffle_render::backend::Context3D>, BitmapError> {
        let texture_label = create_debug_label!("Render target texture");
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let dummy_texture = self
            .descriptors
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: texture_label.as_deref(),
                size: Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::COPY_SRC,
            });

        let handle = BitmapHandle(Arc::new(Texture {
            bind_linear: Default::default(),
            bind_nearest: Default::default(),
            texture: Arc::new(dummy_texture),
            texture_offscreen: Default::default(),
            width: 0,
            height: 0,
        }));
        Ok(Box::new(WgpuContext3D::new(
            self.descriptors.clone(),
            handle,
        )))
    }

    fn context3d_present<'gc>(
        &mut self,
        context: &mut dyn Context3D,
        commands: Vec<Context3DCommand<'gc>>,
        mc: MutationContext<'gc, '_>,
    ) -> Result<(), BitmapError> {
        let context = context
            .as_any_mut()
            .downcast_mut::<WgpuContext3D>()
            .unwrap();
        context.present(commands, mc);
        Ok(())
    }

    fn viewport_dimensions(&self) -> ViewportDimensions {
        ViewportDimensions {
            width: self.target.width(),
            height: self.target.height(),
            scale_factor: self.viewport_scale_factor,
        }
    }

    fn register_shape(
        &mut self,
        shape: DistilledShape,
        bitmap_source: &dyn BitmapSource,
    ) -> ShapeHandle {
        let handle = ShapeHandle(self.meshes.len());
        let mesh = self.register_shape_internal(shape, bitmap_source);
        self.meshes.push(mesh);
        handle
    }

    fn replace_shape(
        &mut self,
        shape: DistilledShape,
        bitmap_source: &dyn BitmapSource,
        handle: ShapeHandle,
    ) {
        let mesh = self.register_shape_internal(shape, bitmap_source);
        self.meshes[handle.0] = mesh;
    }

    fn register_glyph_shape(&mut self, glyph: &swf::Glyph) -> ShapeHandle {
        let shape = ruffle_render::shape_utils::swf_glyph_to_shape(glyph);
        let handle = ShapeHandle(self.meshes.len());
        let mesh = self.register_shape_internal(
            (&shape).into(),
            &ruffle_render::backend::null::NullBitmapSource,
        );
        self.meshes.push(mesh);
        handle
    }

    fn submit_frame(&mut self, clear: Color, commands: CommandList) {
        let frame_output = match self.target.get_next_texture() {
            Ok(frame) => frame,
            Err(e) => {
                log::warn!("Couldn't begin new render frame: {}", e);
                // Attempt to recreate the swap chain in this case.
                self.target.resize(
                    &self.descriptors.device,
                    self.target.width(),
                    self.target.height(),
                );
                return;
            }
        };

        let command_buffers = self.surface.draw_commands(
            frame_output.view(),
            Some(wgpu::Color {
                r: f64::from(clear.r) / 255.0,
                g: f64::from(clear.g) / 255.0,
                b: f64::from(clear.b) / 255.0,
                a: f64::from(clear.a) / 255.0,
            }),
            &self.descriptors,
            &mut self.globals,
            &mut self.uniform_buffers_storage,
            &self.meshes,
            commands,
        );

        self.target.submit(
            &self.descriptors.device,
            &self.descriptors.queue,
            command_buffers,
            frame_output,
        );
    }

    fn register_bitmap(&mut self, bitmap: Bitmap) -> Result<BitmapHandle, BitmapError> {
        if bitmap.width() > self.descriptors.limits.max_texture_dimension_2d
            || bitmap.height() > self.descriptors.limits.max_texture_dimension_2d
        {
            return Err(BitmapError::TooLarge);
        }

        let bitmap = bitmap.to_rgba();
        let extent = wgpu::Extent3d {
            width: bitmap.width(),
            height: bitmap.height(),
            depth_or_array_layers: 1,
        };

        let texture_label = create_debug_label!("Bitmap");
        let texture = self
            .descriptors
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: texture_label.as_deref(),
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_SRC,
            });

        self.descriptors.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Default::default(),
                aspect: wgpu::TextureAspect::All,
            },
            bitmap.data(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * extent.width),
                rows_per_image: None,
            },
            extent,
        );

        let handle = BitmapHandle(Arc::new(Texture {
            texture: Arc::new(texture),
            bind_linear: Default::default(),
            bind_nearest: Default::default(),
            texture_offscreen: Default::default(),
            width: bitmap.width(),
            height: bitmap.height(),
        }));

        Ok(handle)
    }

    fn update_texture(
        &mut self,
        handle: &BitmapHandle,
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    ) -> Result<(), BitmapError> {
        let texture = as_texture(handle);

        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        self.descriptors.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture.texture,
                mip_level: 0,
                origin: Default::default(),
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * extent.width),
                rows_per_image: None,
            },
            extent,
        );

        Ok(())
    }

    fn render_offscreen(
        &mut self,
        handle: BitmapHandle,
        width: u32,
        height: u32,
        commands: CommandList,
    ) -> Result<Bitmap, ruffle_render::error::Error> {
        let texture = as_texture(&handle);

        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // We will (presumably) never render to the majority of textures, so
        // we lazily create the buffer and depth texture.
        // Once created, we never destroy this data, under the assumption
        // that the SWF will try to render to this more than once.
        //
        // If we end up hitting wgpu device limits due to having too
        // many buffers / depth textures rendered at once, we could
        // try storing this data in an LRU cache, evicting entries
        // as needed.
        let texture_offscreen = texture.texture_offscreen.get_or_init(|| {
            let buffer_dimensions = BufferDimensions::new(width as usize, height as usize);
            let buffer_label = create_debug_label!("Render target buffer");
            let buffer = self
                .descriptors
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: buffer_label.as_deref(),
                    size: (buffer_dimensions.padded_bytes_per_row.get() as u64
                        * buffer_dimensions.height as u64),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                    mapped_at_creation: false,
                });
            TextureOffscreen {
                buffer: Arc::new(buffer),
                buffer_dimensions,
                surface: Surface::new(
                    &self.descriptors,
                    DEFAULT_SAMPLE_COUNT,
                    width,
                    height,
                    wgpu::TextureFormat::Rgba8Unorm,
                ),
            }
        });

        let mut target = TextureTarget {
            size: extent,
            texture: texture.texture.clone(),
            format: wgpu::TextureFormat::Rgba8Unorm,
            buffer: texture_offscreen.buffer.clone(),
            buffer_dimensions: texture_offscreen.buffer_dimensions.clone(),
        };

        let (old_width, old_height) = self.globals.resolution();
        self.globals.set_resolution(width, height);

        let frame_output = target
            .get_next_texture()
            .expect("TextureTargetFrame.get_next_texture is infallible");

        let command_buffers = texture_offscreen.surface.draw_commands(
            frame_output.view(),
            None,
            &self.descriptors,
            &mut self.globals,
            &mut self.uniform_buffers_storage,
            &self.meshes,
            commands,
        );
        target.submit(
            &self.descriptors.device,
            &self.descriptors.queue,
            command_buffers,
            frame_output,
        );

        // Capture with premultiplied alpha, which is what we use for all textures
        let image = target.capture(&self.descriptors.device, true);

        let image = image.map(|image| {
            Bitmap::new(
                image.dimensions().0,
                image.dimensions().1,
                ruffle_render::bitmap::BitmapFormat::Rgba,
                image.into_raw(),
            )
        });

        self.globals.set_resolution(old_width, old_height);

        Ok(image.unwrap())
    }
}

// We try to request the highest limits we can get away with
fn required_limits(adapter: &wgpu::Adapter) -> (wgpu::Limits, wgpu::Features) {
    // We start off with the lowest limits we actually need - basically GL-ES 3.0
    let mut limits = wgpu::Limits::downlevel_webgl2_defaults();
    // Then we increase parts of it to the maximum supported by the adapter, to take advantage of
    // more powerful hardware or capabilities
    limits = limits.using_resolution(adapter.limits());
    limits = limits.using_alignment(adapter.limits());

    limits.max_storage_buffers_per_shader_stage =
        adapter.limits().max_storage_buffers_per_shader_stage;
    limits.max_storage_buffer_binding_size = adapter.limits().max_storage_buffer_binding_size;

    let features = wgpu::Features::DEPTH24PLUS_STENCIL8;

    (limits, features)
}
