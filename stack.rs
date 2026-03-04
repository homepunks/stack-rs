use std::marker::PhantomData;
use std::alloc::{alloc, realloc, Layout};

struct DinoStack<T> {
    ptr: *mut T,
    sz: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

trait Stack<T> {
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, val: T);
    fn is_empty(&self) -> bool;
}

impl<T> DinoStack<T> {
    fn new(&self) -> Self {
        DinoStack {
            ptr: std::ptr::null_mut(),
            sz: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_ptr = unsafe {
            let layout = Layout::array::<T>(new_cap).unwrap();
            if self.cap == 0 {
                alloc(layout) as *mut T
            } else {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                realloc(self.ptr as *mut u8, old_layout, layout.size()) as *mut T
            }
        };

        self.ptr = new_ptr;
        self.cap = new_cap;
    }

    fn peek(&self) -> Option<&T> {  
        if self.sz == 0 {
            return None;
        }

        unsafe { Some(&*self.ptr.add(self.sz - 1)) }
    }

    fn push(&mut self, val: T) {
        if self.sz == self.cap {
            self.grow()
        }

        unsafe { self.ptr.add(self.sz).write(val); }

        self.sz += 1
    }

    fn pop(&mut self) -> Option<T> {
        if self.sz == 0 {
            return None;
        }
        self.sz -= 1;

        unsafe { Some(self.ptr.add(self.sz).read()) }
    }
}

fn main() {
    println!("Hellope")
}
