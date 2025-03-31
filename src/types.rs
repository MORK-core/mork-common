use core::cell::UnsafeCell;
use core::ops::{Index, IndexMut};

pub type JustResult = Result<(), ()>;
pub type ResultWithErr<T> = Result<(), T>;
pub type ResultWithValue<T> = Result<T, ()>;

pub struct Array<T: Default, const N: usize>([T; N]);
impl<T: Default, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        let inner = [(); N].map(|_| T::default());
        Array(inner)
    }
}
impl<T: Default, const N: usize> Index<usize> for Array<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Default, const N: usize> IndexMut<usize> for Array<T, N> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub struct SyncUnsafeCell<T> {
    inner: UnsafeCell<T>,
}
unsafe impl<T> Sync for SyncUnsafeCell<T> where T: Sync {}

impl<T> SyncUnsafeCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// 获取可变引用（必须在外部锁的保护下调用！）
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn get(&self) -> &T {
        unsafe { & *self.inner.get() }
    }
}