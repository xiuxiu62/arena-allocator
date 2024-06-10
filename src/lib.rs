#[derive(Debug)]
pub struct Arena<const CAPACITY: usize, const ALIGNMENT: usize> {
    data: *mut u8,
    offset: usize,
    layout: std::alloc::Layout,
}

impl<const CAPACITY: usize, const ALIGNMENT: usize> Arena<CAPACITY, ALIGNMENT> {
    // SAFETY: this function is only unsafe if the caller supplies a capacity or alignment of zero
    pub fn new() -> ArenaResult<Self> {
        let layout = std::alloc::Layout::from_size_align(CAPACITY, ALIGNMENT)?;
        let data = unsafe { std::alloc::alloc(layout) };

        // Zero-initialize the memory
        unsafe {
            std::ptr::write_bytes(data, 0, CAPACITY);
        }

        Ok(Self {
            data,
            offset: 0,
            layout,
        })
    }

    // SAFETY: we've ensured that we have enough remaining capacity, prior to allocating memory
    pub fn allocate<'region, T>(&mut self) -> ArenaResult<&'region mut T> {
        let size = std::mem::size_of::<T>();
        if self.offset + size > CAPACITY {
            return Err(ArenaError::Alloc(AllocError));
        }

        let raw = unsafe { self.data.add(self.offset) };
        self.offset += size;

        unsafe { raw.cast::<T>().as_mut() }.ok_or(ArenaError::Alloc(AllocError))
    }

    // SAFETY: we're iterating only over the allocated capacity of our data
    pub fn dump(&self) -> ArenaResult<String> {
        (0..CAPACITY).try_fold("".to_owned(), |mut acc, i| -> ArenaResult<String> {
            let value = unsafe { self.data.add(i).as_ref() }.ok_or(ArenaError::Alloc(AllocError));

            acc.push_str(&format!("{:02x} ", value.unwrap()));
            if (i + 1) % 16 == 0 {
                acc.push('\n');
            }

            Ok(acc)
        })
    }
}

// SAFETY: we're abiding by the same layout used in our parent allocation
impl<const CAPACITY: usize, const ALIGNMENT: usize> Drop for Arena<CAPACITY, ALIGNMENT> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data, self.layout);
        }
    }
}

pub type ArenaResult<T> = Result<T, ArenaError>;

#[derive(Debug)]
pub enum ArenaError {
    Alloc(AllocError),
    Layout(std::alloc::LayoutError),
}

impl From<AllocError> for ArenaError {
    fn from(value: AllocError) -> Self {
        Self::Alloc(value)
    }
}

impl From<std::alloc::LayoutError> for ArenaError {
    fn from(value: std::alloc::LayoutError) -> Self {
        Self::Layout(value)
    }
}

impl std::fmt::Display for ArenaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
pub struct AllocError;

impl std::fmt::Display for AllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AllocError {}
