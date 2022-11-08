use bumpalo::boxed::Box as BumpBox;
use bumpalo::Bump;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::{null, null_mut};

pub struct BumpSlab<T: Sized> {
    bump: Bump,
    next: Cell<*mut ()>,
    _p: PhantomData<T>,
}

impl<'a, T> BumpSlab<T> {
    pub fn new() -> Self {
        Self {
            bump: Bump::new(),
            next: Cell::new(null_mut()),
            _p: PhantomData,
        }
    }

    /// By default, you need to remove the slot by calling "remove". Otherwise you will be leaking memory
    pub fn push(&'a self, value: T) -> Slot<'a, T> {
        let current = self.next.get();

        if current.is_null() {
            return Slot(self.bump.alloc(SlotInner {
                value: ManuallyDrop::new(value),
            }));
        }

        let available = unsafe { &mut *(current as *mut SlotInner<T>) };
        unsafe {
            if available.next.is_null() {
                self.next.set(null_mut());
            } else {
                self.next.set(available.next as *mut ());
            }
        }

        available.value = ManuallyDrop::new(value);
        Slot(available)
    }

    pub fn bump(&self) -> &Bump {
        &self.bump
    }

    pub fn remove(&'a self, slot: Slot<'a, T>) {
        // Drop the value
        unsafe { ManuallyDrop::drop(&mut slot.0.value) };

        let next = self.next.get();

        // If no head, set it
        if next.is_null() {
            self.next.set(slot.0 as *mut SlotInner<T> as *mut ());
            return;
        }

        // Assign the next item in the linked list
        // point this slot to the head, and then the bumpslab head to the new slot
        let next = unsafe { &mut *(next as *mut SlotInner<T>) };
        slot.0.next = next;

        self.next.set(slot.0 as *mut SlotInner<T> as *mut ());
    }
}

pub struct Slot<'a, T>(&'a mut SlotInner<T>);

pub union SlotInner<T> {
    value: ManuallyDrop<T>,
    next: *mut SlotInner<T>,
}

impl<T> Slot<'_, T> {
    pub fn ptr(&self) -> *const T {
        unsafe { &*self.0.value }
    }
}

impl<T> std::ops::Deref for SlotInner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<T> std::ops::DerefMut for SlotInner<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value }
    }
}
