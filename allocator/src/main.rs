#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::ptr;

struct SimpleAllocator {
    heap_start: *mut u8,
    heap_end: *mut u8,
    current: *mut u8,
}

impl SimpleAllocator {
    // Créé un nouvel allocateur avec un bloc de mémoire pré-alloué
    pub const fn new(heap_start: *mut u8, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: unsafe { heap_start.add(heap_size) },
            current: heap_start,
        }
    }

    // Alloue une taille donnée en mémoire
    pub unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        if self.current.add(size) > self.heap_end {
            ptr::null_mut() // Si on dépasse la mémoire, retourne un pointeur nul
        } else {
            let allocated = self.current;
            self.current = self.current.add(size);
            allocated
        }
    }

    // Remet à zéro l'allocateur pour réutiliser la mémoire
    pub unsafe fn reset(&mut self) {
        self.current = self.heap_start;
    }
}




#[no_mangle]
pub extern "C" fn _start() -> ! {

    // Exemple d'utilisation
    const HEAP_SIZE: usize = 1024;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    let mut allocator = unsafe { SimpleAllocator::new(HEAP.as_mut_ptr(), HEAP_SIZE) };

    unsafe {
        let mem1 = allocator.alloc(128);
        if !mem1.is_null() {
        }

        let mem2 = allocator.alloc(256);
        if !mem2.is_null() {
        }

        // Réinitialise l'allocateur pour réutiliser toute la mémoire
        allocator.reset();
    }

    loop{}
  
}



///Rust needs a panic handler for now just a no return function that indefinitely loop 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{
    }
}
/*
fn main(){}
*/
