pub mod swappers;
pub mod frame;

pub const SWAPPER_DEFAULT_CAPACITY: usize = 128;

pub trait Swapper<T> {
    /// reserve the capacity of memory (number of pages)
    fn reserve(&mut self, capacity: usize);

    /// refer a page
    ///
    /// returns `Ok(())` if the page hit.
    /// otherwise returns the page that was swapped out
    fn refer(&mut self, page: T) -> Result<(), Option<T>>;
}