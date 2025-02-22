#![no_std]

use core::ptr::{self, NonNull};
use core::mem;
use alloc::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::Heap;

/// Taille des slabs prédéfinis
const SLAB_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

/// Un noeud de liste pour gérer les blocs libres
type FreeList = Option<&'static mut ListNode>;

struct ListNode {
    next: FreeList,
}

pub struct SlabAllocator {
    slabs: [FreeList; SLAB_SIZES.len()],
    fallback: Heap,
}

impl SlabAllocator {
    /// Création d'un allocateur de slab vide
    pub const fn new() -> Self {
        const EMPTY: FreeList = None;
        SlabAllocator {
            slabs: [EMPTY; SLAB_SIZES.len()],
            fallback: Heap::empty(),
        }
    }

    /// Initialise l'allocateur avec une zone mémoire
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback.init(heap_start, heap_size);
    }

    /// Trouve l'index du slab adapté
    fn slab_index(layout: &Layout) -> Option<usize> {
        let required_size = layout.size().max(layout.align());
        SLAB_SIZES.iter().position(|&size| size >= required_size)
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self as *const _ as *mut SlabAllocator; // Cast mutable
        match Self::slab_index(&layout) {
            Some(index) => {
                let slab = &mut (*allocator).slabs[index];
                match slab.take() {
                    Some(node) => {
                        (*allocator).slabs[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        let size = SLAB_SIZES[index];
                        let align = size;
                        let new_layout = Layout::from_size_align(size, align).unwrap();
                        (*allocator).fallback.allocate_first_fit(new_layout).map_or(ptr::null_mut(), |ptr| ptr.as_ptr())
                    }
                }
            }
            None => (*allocator).fallback.allocate_first_fit(layout).map_or(ptr::null_mut(), |ptr| ptr.as_ptr()),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self as *const _ as *mut SlabAllocator;
        match Self::slab_index(&layout) {
            Some(index) => {
                let new_node = ListNode { next: (*allocator).slabs[index].take() };
                assert!(mem::size_of::<ListNode>() <= SLAB_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= SLAB_SIZES[index]);
                let node_ptr = ptr as *mut ListNode;
                node_ptr.write(new_node);
                (*allocator).slabs[index] = Some(&mut *node_ptr);
            }
            None => {
                let non_null_ptr = NonNull::new(ptr).unwrap();
                (*allocator).fallback.deallocate(non_null_ptr, layout);
            }
        }
    }
}
