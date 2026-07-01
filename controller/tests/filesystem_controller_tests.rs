use controller::filesystem_controller::{FilesystemController, ProfilingFlash};

struct MockFlash {
    data: [u8; 1024 * 64],
}

impl MockFlash {
    fn new() -> Self {
        Self {
            data: [0xFF; 1024 * 64],
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for MockFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for MockFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for MockFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.data[from as usize..to as usize].fill(0xFF);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for MockFlash {}

#[test]
fn test_filesystem_controller_flow() {
    futures::executor::block_on(async {
        let flash = MockFlash::new();
        let profiling_flash = ProfilingFlash::new(flash);
        let mut fs = FilesystemController::new(profiling_flash, 0..1024 * 64);

        // Verify erase profiling works
        assert_eq!(fs.flash.erase_count(), 0);
        // Trigger a manual page erase via the controller's flash interface
        use embedded_storage_async::nor_flash::NorFlash;
        assert!(fs.flash.erase(0, 1024).await.is_ok());
        assert_eq!(fs.flash.erase_count(), 1);

        // Initially no files
        let mut buf = [0u8; 128];
        assert_eq!(fs.list_files(&mut buf).await.unwrap(), None);

        // Write a file
        fs.write_file("test.txt", b"hello world").await.unwrap();

        // Read the file
        let content = fs.read_file("test.txt", &mut buf).await.unwrap().unwrap();
        assert_eq!(content, b"hello world");

        // List files
        let mut list_buf = [0u8; 128];
        let list = fs.list_files(&mut list_buf).await.unwrap().unwrap();
        assert_eq!(list, b"test.txt");

        // Write a second file
        fs.write_file("second.txt", b"another file").await.unwrap();

        // List files again
        let list = fs.list_files(&mut list_buf).await.unwrap().unwrap();
        assert!(list == b"test.txt\nsecond.txt" || list == b"second.txt\ntest.txt");

        // Remove a file
        fs.remove_file("test.txt").await.unwrap();

        // List files after removal
        let list = fs.list_files(&mut list_buf).await.unwrap().unwrap();
        assert_eq!(list, b"second.txt");

        // Read removed file returns None
        assert_eq!(fs.read_file("test.txt", &mut buf).await.unwrap(), None);
    });
}
