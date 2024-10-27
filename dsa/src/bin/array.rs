use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::{self, NonNull};
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct DynamicArray<T> {
    ptr: NonNull<T>,  // Pointer to the array data in heap memory
    len: usize,       // Current number of elements in the array
    capacity: usize,  // Allocated size of the array
}

impl<T> DynamicArray<T> {
    /// Creates a new, empty DynamicArray with an initial capacity
    pub fn new() -> Self {
        let capacity = 4; // Start with a small initial capacity
        let ptr = unsafe {
            let layout = Layout::array::<T>(capacity).expect("Layout creation failed");
            NonNull::new(alloc(layout) as *mut T).expect("Allocation failed")
        };
        Self { ptr, len: 0, capacity }
    }

    /// Adds an element to the end of the array, resizing if needed
    pub fn push(&mut self, element: T) {
        if self.len == self.capacity {
            self.resize();
        }
        unsafe {
            // Write element at the next available slot
            ptr::write(self.ptr.as_ptr().add(self.len), element);
        }
        self.len += 1;
    }

    /// Removes the last element from the array and returns it
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe {
            // Read the value and leave the memory uninitialized
            Some(ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }

    /// Gets the length of the array
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the capacity of the array
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Resizes the array by doubling its capacity
    fn resize(&mut self) {
        self.capacity *= 2;
        let new_layout = Layout::array::<T>(self.capacity).expect("Layout creation failed");
        unsafe {
            let new_ptr = realloc(self.ptr.as_ptr() as *mut u8, new_layout, self.capacity * std::mem::size_of::<T>()) as *mut T;
            self.ptr = NonNull::new(new_ptr).expect("Reallocation failed");
        }
    }
}

// Implement Drop trait to free the allocated memory when the array goes out of scope
impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        // Call drop on each element if it implements Drop
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
        }
        // Free the allocated memory
        let layout = Layout::array::<T>(self.capacity).expect("Layout creation failed");
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

// Implement Index trait for accessing elements in the array
impl<T> Index<usize> for DynamicArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Index out of bounds");
        unsafe { &*self.ptr.as_ptr().add(index) }
    }
}

// Implement IndexMut trait for mutable access to elements
impl<T> IndexMut<usize> for DynamicArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len, "Index out of bounds");
        unsafe { &mut *self.ptr.as_ptr().add(index) }
    }
}

fn main() {
    let mut array = DynamicArray::new();

    // Pushing elements to the array
    array.push(1);
    array.push(2);
    array.push(3);
    array.push(4);
    array.push(5); // Triggers a resize

    println!("Array length: {}", array.len());
    println!("Array capacity: {}", array.capacity());

    // Accessing elements
    for i in 0..array.len() {
        println!("Element {}: {}", i, array[i]);
    }

    // Popping elements from the array
    while let Some(value) = array.pop() {
        println!("Popped value: {}", value);
    }
}
