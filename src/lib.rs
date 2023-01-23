use bumpalo::Bump;
use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

pub struct BumpSlab<T: Sized> {
    bump: Bump,
    next: Cell<Option<NonNull<SlotInner<T>>>>,
    _p: PhantomData<T>,
}

impl<'a, T> BumpSlab<T> {
    pub fn new() -> Self {
        Self {
            bump: Bump::new(),
            next: Cell::new(None),
            _p: PhantomData,
        }
    }

    /// By default, you need to remove the slot by calling "remove". Otherwise you will be leaking memory
    pub fn push(&'a self, value: T) -> Slot<'a, T> {
        let current = self.next.get();

        match current {
            None => Slot(self.bump.alloc(SlotInner {
                value: ManuallyDrop::new(UnsafeCell::new(value)),
            })),
            Some(mut current) => {
                let available = unsafe { current.as_mut() };
                unsafe {
                    self.next.set(available.next);
                }

                available.value = ManuallyDrop::new(UnsafeCell::new(value));
                Slot(available)
            }
        }
    }

    /// Acquire the internal bump allocator
    pub fn bump(&self) -> &Bump {
        &self.bump
    }

    pub fn remove(&'a self, slot: Slot<'a, T>) {
        // Drop the value
        unsafe { ManuallyDrop::drop(&mut slot.0.value) };

        let next = self.next.get();

        if let Some(next) = next {
            // Assign the next item in the linked list
            // point this slot to the head, and then the bumpslab head to the new slot
            slot.0.next = Some(next);
        }
        self.next.set(Some(slot.0.into()));
    }
}

/// A keyed container for the BumpSlab
pub struct Slot<'a, T>(&'a mut SlotInner<T>);

/// The inner is a union between the value and the next item in the linked list
///
/// This forms a intruisive linked list which let us chase down free spots as they become available
union SlotInner<T> {
    value: ManuallyDrop<UnsafeCell<T>>,
    next: Option<NonNull<SlotInner<T>>>,
}

impl<T> Slot<'_, T> {
    pub fn ptr(&self) -> *const T {
        unsafe { self.0.value.get() }
    }

    pub fn ptr_mut(&self) -> *mut T {
        unsafe { self.0.value.get() }
    }
}

impl<T> std::ops::Deref for SlotInner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.get() }
    }
}

impl<T> std::ops::DerefMut for SlotInner<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.get_mut() }
    }
}
