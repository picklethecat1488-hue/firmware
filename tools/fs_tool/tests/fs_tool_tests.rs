use embedded_storage_async::nor_flash::{NorFlash, ReadNorFlash};
use fs_tool::{string_to_key, DataType, HostFlash};

#[test]
fn test_string_to_key() {
    // Exact 32 bytes
    let key32 = string_to_key("abcdefghijklmnopqrstuvwxyz123456");
    assert_eq!(&key32[..], b"abcdefghijklmnopqrstuvwxyz123456");

    // Short string (should pad with 0)
    let key_short = string_to_key("hello");
    assert_eq!(&key_short[..5], b"hello");
    assert_eq!(key_short[5], 0);

    // Truncated string
    let key_long = string_to_key("abcdefghijklmnopqrstuvwxyz123456789");
    assert_eq!(&key_long[..], b"abcdefghijklmnopqrstuvwxyz123456");
}

#[test]
fn test_data_type_decoding() {
    assert_eq!(
        DataType::from_filename("calibration.bin"),
        DataType::Calibration
    );
    assert_eq!(
        DataType::from_filename("telemetry.rrd"),
        DataType::Telemetry
    );
    assert_eq!(DataType::from_filename("crash.log"), DataType::CrashLog);
    assert_eq!(DataType::from_filename("unknown.data"), DataType::Unknown);

    assert_eq!(DataType::Calibration.to_str(), "Protobuf Calibration Data");
    assert_eq!(DataType::Telemetry.to_str(), "CBOR Telemetry Data");
}

#[test]
fn test_host_flash_driver() {
    futures::executor::block_on(async {
        let mut flash = HostFlash::new(vec![0xFF; 2048]);
        assert_eq!(flash.capacity(), 2048);

        // Write some bytes
        let write_data = [0xAA, 0xBB, 0xCC, 0xDD];
        assert!(flash.write(1024, &write_data).await.is_ok());

        // Read back
        let mut read_data = [0u8; 4];
        assert!(flash.read(1024, &mut read_data).await.is_ok());
        assert_eq!(read_data, write_data);

        // Erase sector
        assert!(flash.erase(1024, 2048).await.is_ok());
        assert!(flash.read(1024, &mut read_data).await.is_ok());
        assert_eq!(read_data, [0xFF; 4]);

        // Out of bounds checks
        assert!(flash.read(2047, &mut read_data).await.is_err());
        assert!(flash.write(2047, &write_data).await.is_err());
    });
}
