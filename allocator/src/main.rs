#![cfg_attr(not(feature = "test"), no_std)]
#![cfg_attr(not(feature = "test"), no_main)]

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

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, size: usize) {
        // Vérifie si le pointeur est dans la plage du tas
        if ptr >= self.heap_start && ptr < self.heap_end {
            // Remet les octets de la mémoire à zéro
            for i in 0..size {
                ptr.add(i).write(0);
            }
            // Si le pointeur correspond au bloc le plus récent alloué,
            // recule le curseur pour permettre la réutilisation de cette mémoire
            if ptr.add(size) == self.current {
                self.current = ptr;
            }
        }
    }

    // Réinitialise l'allocateur pour réutiliser la mémoire
    pub unsafe fn reset(&mut self) {
        self.current = self.heap_start;
    }
}

#[cfg_attr(not(feature = "test"), no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Gestionnaire de panic pour le compilateur Rust
#[cfg_attr(not(feature = "test"), panic_handler)]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "test")]
fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_and_copy() {
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

        let mut allocator = unsafe { SimpleAllocator::new(HEAP.as_mut_ptr(), HEAP_SIZE) };

        unsafe {
            // Données à tester
            let data = [10, 20, 30, 40, 50];
            
            // Alloue et copie les données
            let mem = allocator.alloc_and_copy(&data);
            
            // Vérifie que la mémoire est allouée
            assert!(!mem.is_null(), "Allocation échouée!");

            // Vérifie les données copiées
            if let Some(allocated_data) = allocator.read(mem, data.len()) {
                assert_eq!(allocated_data, &data, "Les données ne correspondent pas!");
            } else {
                panic!("Lecture échouée de la mémoire allouée!");
            }

            // Réinitialise et vérifie qu'on peut réutiliser la mémoire
            allocator.reset();
            let new_mem = allocator.alloc(data.len());
            assert_eq!(new_mem, mem, "Mémoire non réutilisable après réinitialisation!");
        }
    }

    #[test]
    fn test_out_of_memory() {
        const HEAP_SIZE: usize = 16;
        static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

        let mut allocator = unsafe { SimpleAllocator::new(HEAP.as_mut_ptr(), HEAP_SIZE) };

        unsafe {
            // Alloue plus de mémoire que disponible
            let mem = allocator.alloc(HEAP_SIZE + 1);
            assert!(mem.is_null(), "L'allocation aurait dû échouer!");
        }
    }
}

/*
fn main(){}
*/
