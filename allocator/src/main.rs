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
    // Crée un nouvel allocateur avec un bloc de mémoire pré-alloué
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

    // Alloue de la mémoire et copie les données dedans
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

    // Méthode pour lire les données à une adresse spécifique
    pub unsafe fn reading(&self, ptr: *const u8, size: usize) -> Option<&[u8]> {
        if ptr.is_null() || ptr.add(size) > self.heap_end {
            None
        } else {
            Some(core::slice::from_raw_parts(ptr, size))
        }
    }

    // Réinitialise l'allocateur pour réutiliser la mémoire
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
        // Données à copier
        let data = [1, 2, 3, 4, 5];
        
        // Alloue et copie les données
        let mem = allocator.alloc_and_copy(&data);
        // Récupère et affiche les données copiées
        if let Some(allocated_data) = allocator.reading(mem, data.len()) {
            for &byte in allocated_data {
                // Utilisez la donnée (ici, vous pouvez afficher avec un débogueur ou effectuer d'autres opérations)
                // Affichage fictif pour éviter les opérations réelles dans un contexte `no_std`
            }
        }
        // Réinitialise l'allocateur pour réutiliser toute la mémoire
        allocator.reset();
    }

    loop{}
}

/// Gestionnaire de panic pour le compilateur Rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*
fn main(){}
*/
