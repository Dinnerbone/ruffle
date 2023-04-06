use crate::{ColorAdjustments, Transforms};
use bytemuck::Pod;
use ouroboros::self_referencing;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::{marker::PhantomData, mem};
use typed_arena::Arena;
use wgpu::util::StagingBelt;

#[self_referencing]
pub struct BufferStorage<T: Pod> {
    phantom: PhantomData<T>,
    arena: Arena<Block>,

    #[borrows(arena)]
    #[not_covariant]
    allocator: RefCell<Allocator<'this>>,

    staging_belt: RefCell<StagingBelt>,
    aligned_uniforms_size: u32,
    cur_block: usize,
    cur_offset: u32,
}

struct Allocator<'a> {
    arena: &'a Arena<Block>,
    blocks: Vec<&'a Block>,
}

impl<T: Pod> BufferStorage<T> {
    /// The size of each block.
    /// Uniforms are copied into each block until it reaches capacity, at which point a new
    /// block will be allocated.
    pub const BLOCK_SIZE: u32 = 65536;

    /// The uniform data size for a single draw call.
    pub const UNIFORMS_SIZE: u64 = mem::size_of::<T>() as u64;

    pub fn from_alignment(uniform_alignment: u32) -> Self {
        // Calculate alignment of uniforms.
        let align_mask = uniform_alignment - 1;
        let aligned_uniforms_size = (Self::UNIFORMS_SIZE as u32 + align_mask) & !align_mask;
        BufferStorageBuilder {
            arena: Arena::with_capacity(8),
            allocator_builder: |arena| {
                RefCell::new(Allocator {
                    arena,
                    blocks: Vec::with_capacity(8),
                })
            },
            staging_belt: RefCell::new(StagingBelt::new(u64::from(Self::BLOCK_SIZE) / 2)),
            aligned_uniforms_size,
            phantom: PhantomData,
            cur_block: 0,
            cur_offset: 0,
        }
        .build()
    }

    /// Adds a newly allocated buffer to the block list, and returns it.
    pub fn allocate_block(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) {
        let buffer_label = create_debug_label!("Dynamic buffer");
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: buffer_label.as_deref(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: Self::BLOCK_SIZE.into(),
            mapped_at_creation: false,
        });

        let bind_group_label = create_debug_label!("Dynamic buffer bind group");
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: bind_group_label.as_deref(),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(std::mem::size_of::<T>() as u64),
                }),
            }],
        });

        self.with_allocator(|alloc| {
            let mut alloc = alloc.borrow_mut();
            let block = alloc.arena.alloc(Block { buffer, bind_group });
            alloc.blocks.push(block);
        });
    }

    pub fn recall(&mut self) {
        self.with_staging_belt(|belt| belt.borrow_mut().recall());
        self.with_cur_block_mut(|v| *v = 0);
        self.with_cur_offset_mut(|v| *v = 0);
    }

    /// Should be called at the end of a frame.
    pub fn finish(&mut self) {
        self.with_staging_belt(|belt| belt.borrow_mut().finish());
    }

    // /// Enqueue `data` for upload into the given command encoder, and set the bind group on `render_pass`
    // /// to use the uniform data.
    // pub fn write_uniforms<'a, 'b>(
    //     &'a mut self,
    //     device: &wgpu::Device,
    //     layout: &wgpu::BindGroupLayout,
    //     command_encoder: &mut wgpu::CommandEncoder,
    //     render_pass: &mut wgpu::RenderPass<'b>,
    //     bind_group_index: u32,
    //     data: &T,
    // ) where
    //     'a: 'b,
    // {
    //     // Allocate a new block if we've exceeded our capacity.
    //     if *self.borrow_cur_block() >= self.with_allocator(|alloc| alloc.borrow().blocks.len()) {
    //         self.allocate_block(device, layout);
    //     }
    //
    //     let block: &Block =
    //         self.with_allocator(|alloc| alloc.borrow().blocks[*self.borrow_cur_block()]);
    //
    //     // Copy the data into the buffer via the staging belt.
    //     self.with_staging_belt(|belt| {
    //         belt.borrow_mut()
    //             .write_buffer(
    //                 command_encoder,
    //                 &block.buffer,
    //                 (*self.borrow_cur_offset()).into(),
    //                 BufferStorage::<T>::UNIFORMS_SIZE.try_into().unwrap(),
    //                 device,
    //             )
    //             .copy_from_slice(bytemuck::cast_slice(std::slice::from_ref(data)));
    //     });
    //
    //     // Set the bind group to the final uniform location.
    //     render_pass.set_bind_group(
    //         bind_group_index,
    //         &block.bind_group,
    //         &[*self.borrow_cur_offset()],
    //     );
    //
    //     // Advance offset.
    //     self.with_cur_offset_mut(|v| *v += self.borrow_aligned_uniforms_size());
    //     // Advance to next buffer if we are out of room in this buffer.
    //     if BufferStorage::<T>::BLOCK_SIZE - *self.borrow_cur_offset()
    //         < *self.borrow_aligned_uniforms_size()
    //     {
    //         self.with_cur_block_mut(|v| *v += 1);
    //         self.with_cur_offset_mut(|v| *v = 0);
    //     }
    // }
}

pub struct StagingBuffersStorage {
    pub uniforms: BufferStorage<Transforms>,
    pub colors: BufferStorage<ColorAdjustments>,
}

impl Debug for StagingBuffersStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StagingBuffersStorage").finish()
    }
}

impl StagingBuffersStorage {
    pub fn from_alignment(alignment: u32) -> Self {
        Self {
            uniforms: BufferStorage::from_alignment(alignment),
            colors: BufferStorage::from_alignment(alignment),
        }
    }

    pub fn recall(&mut self) {
        self.uniforms.recall();
        self.colors.recall();
    }

    pub fn finish(&mut self) {
        self.uniforms.finish();
        self.colors.finish();
    }
}

/// A block of GPU memory that will contain our uniforms.
#[derive(Debug)]
#[allow(dead_code)]
struct Block {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}
