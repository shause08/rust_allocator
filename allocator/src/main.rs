#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;

/// Structure de l'allocateur
struct SimpleAllocator {
    heap_start: *mut u8,
    heap_end: *mut u8,
    current: *mut u8,
}

impl SimpleAllocator {
    /// Création d'un nouvel allocateur
    pub const fn new(heap_start: *mut u8, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: unsafe { heap_start.add(heap_size) },
            current: heap_start,
        }
    }
    /// Allocation de la mémoire
    pub unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        if self.current.add(size) > self.heap_end {
            /// Si on dépasse la mémoire, retourne un pointeur nul
            ptr::null_mut() 
        } else {
            let allocated = self.current;
            self.current = self.current.add(size);
            allocated
        }
    }
    pub unsafe fn alloc_and_copy(&mut self, data: &[u8]) -> *mut u8 {
        let size = data.len();
        let dest = self.alloc(size);
        if !dest.is_null() {
            for i in 0..size {
                *dest.add(i) = data[i];
            }
        }
        dest
    }
    /// Permet de savoir la donnée qui a était alloué
    pub unsafe fn reading(&self, ptr: *const u8, size: usize) -> Option<&[u8]> {
        if ptr.is_null() || ptr.add(size) > self.heap_end {
            None
        } else {
            Some(core::slice::from_raw_parts(ptr, size))
        }
    }
    /// Désalloc la donnée
    pub unsafe fn dealloc(&mut self, ptr: *mut u8, size: usize) {
        if ptr >= self.heap_start && ptr < self.heap_end {
            for i in 0..size {
                ptr.add(i).write(0);
            }
            if ptr.add(size) == self.current {
                self.current = ptr;
            }
        }
    }
    /// Désalloc la mémoire
    pub unsafe fn reset(&mut self) {
        self.current = self.heap_start;
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    const HEAP_SIZE: usize = 1024;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        /// Création de l'allocateur
        let mut allocator = SimpleAllocator::new(HEAP.as_mut_ptr(), HEAP_SIZE);

        unsafe {
            /// Test de la fonction alloc
            let mem1 = allocator.alloc(128);
            assert!(!mem1.is_null(), "Erreur d'allocation de mémoire");
        
            /// Test de la fonction alloc_and_copy
            let data = [1, 2, 3, 4, 5];
            let mem2 = allocator.alloc_and_copy(&data);
            assert!(!mem2.is_null(), "Erreur d'allocation et de copie");
        
            for i in 0..data.len() {
                assert_eq!(*mem2.add(i), data[i], "Erreur dans la copie des données");
            }
        
            /// Test de la fonction reading
            if let Some(read_data) = allocator.reading(mem2, data.len()) {
                assert_eq!(read_data, &data, "Données lues incorrectes");
            } else {
                panic!("Lecture échouée !");
            }
        
            /// Test de la fonction dealloc
            allocator.dealloc(mem2, data.len());
            for i in 0..data.len() {
                assert_eq!(*mem2.add(i), 0, "Erreur lors de la désallocation");
            }
        
            /// Test de la fonction reset
            allocator.reset();
            assert_eq!(allocator.current, allocator.heap_start, "Échec du reset de l'allocateur");
        }    
    }
    loop {}
}

/// Fonction de gestion de la panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_log("Panique déclenchée !");
    }
    loop {}
}
