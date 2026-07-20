//! NOR Flash driver adapters.

#![deny(missing_docs)]

/// Adapter exposing a blocking nor-flash driver as an asynchronous nor-flash driver.
pub struct BlockingAsyncFlash<F>(pub F);

impl<F: embedded_storage::nor_flash::ErrorType> embedded_storage_async::nor_flash::ErrorType
    for BlockingAsyncFlash<F>
{
    type Error = F::Error;
}

impl<F: embedded_storage::nor_flash::ReadNorFlash> embedded_storage_async::nor_flash::ReadNorFlash
    for BlockingAsyncFlash<F>
{
    const READ_SIZE: usize = F::READ_SIZE;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::ReadNorFlash::read(&mut inner, offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<F: embedded_storage::nor_flash::NorFlash> embedded_storage_async::nor_flash::NorFlash
    for BlockingAsyncFlash<F>
{
    const WRITE_SIZE: usize = F::WRITE_SIZE;
    const ERASE_SIZE: usize = F::ERASE_SIZE;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::NorFlash::write(&mut inner, offset, bytes)
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::NorFlash::erase(&mut inner, from, to)
    }
}

impl<F: embedded_storage::nor_flash::NorFlash> embedded_storage_async::nor_flash::MultiwriteNorFlash
    for BlockingAsyncFlash<F>
{
}
