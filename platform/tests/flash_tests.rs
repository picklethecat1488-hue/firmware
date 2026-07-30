use platform::flash::{read_file_direct, write_file_direct, write_telemetry_record_direct};

struct TestFlash {
    data: Vec<u8>,
}

impl embedded_storage_async::nor_flash::ErrorType for TestFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for TestFlash {
    const READ_SIZE: usize = 1;
    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        bytes.copy_from_slice(&self.data[start..end]);
        Ok(())
    }
    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for TestFlash {
    const WRITE_SIZE: usize = 1;
    const ERASE_SIZE: usize = 4096;
    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        self.data[start..end].copy_from_slice(bytes);
        Ok(())
    }
    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start = from as usize;
        let end = to as usize;
        self.data[start..end].fill(0xFF);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for TestFlash {}

#[test]
fn test_direct_file_operations() {
    futures::executor::block_on(async {
        let mut flash = TestFlash {
            data: vec![0xFF; 16 * 1024],
        };
        let range = platform::types::MapFilesystem(0..16 * 1024);
        let mut map_buf = vec![0u8; 4096];

        // Write a file
        let content = b"hello direct file";
        write_file_direct(&mut flash, range.clone(), &mut map_buf, "test.txt", content)
            .await
            .unwrap();

        // Read the file
        let mut out_buf = vec![0u8; 100];
        let len = read_file_direct(
            &mut flash,
            range.clone(),
            &mut map_buf,
            "test.txt",
            &mut out_buf,
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(&out_buf[..len], content);

        // Overwrite the file
        let new_content = b"modified content";
        write_file_direct(
            &mut flash,
            range.clone(),
            &mut map_buf,
            "test.txt",
            new_content,
        )
        .await
        .unwrap();

        // Read modified file
        let len2 = read_file_direct(
            &mut flash,
            range.clone(),
            &mut map_buf,
            "test.txt",
            &mut out_buf,
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(&out_buf[..len2], new_content);
    });
}

#[test]
fn test_direct_telemetry_operations() {
    futures::executor::block_on(async {
        let mut flash = TestFlash {
            data: vec![0xFF; 16 * 1024],
        };
        let telemetry_range = platform::types::QueueFilesystem(8 * 1024..16 * 1024);

        let rec = model::telemetry::TelemetryRecord::PeripheralError(
            model::types::PeripheralError::DeviceNotFound(0x12),
        );

        // Write record
        write_telemetry_record_direct(&mut flash, telemetry_range.clone(), &rec)
            .await
            .unwrap();

        // Read back record from telemetry queue partition
        let mut cache = sequential_storage::cache::NoCache::new();
        let mut iterator =
            sequential_storage::queue::iter(&mut flash, telemetry_range.0.clone(), &mut cache)
                .await
                .unwrap();
        let mut item_buf = [0u8; model::telemetry::TELEMETRY_RECORD_SIZE];
        let entry = iterator.next(&mut item_buf).await.unwrap().unwrap();
        let decoded =
            model::telemetry::TelemetryRecord::deserialize_from_slice(entry.into_buf()).unwrap();
        assert_eq!(decoded.0, 0); // timestamp
        assert_eq!(decoded.1, rec); // Record
    });
}
