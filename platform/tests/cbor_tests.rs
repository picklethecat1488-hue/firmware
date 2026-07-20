use platform::cbor::{extract_bytes, ConstCborWriter};

#[test]
fn test_cbor_write_u32_boundaries() {
    // 1. Value <= 23 (1 byte)
    for val in [0u32, 5, 23] {
        let writer = ConstCborWriter::<16>::new().write_u32(val);
        let bytes: [u8; 1] = extract_bytes(writer.buf);
        assert_eq!(writer.len, 1);
        let decoded: u32 = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, val);
    }

    // 2. 24 <= Value <= 255 (2 bytes)
    for val in [24u32, 128, 255] {
        let writer = ConstCborWriter::<16>::new().write_u32(val);
        let bytes: [u8; 2] = extract_bytes(writer.buf);
        assert_eq!(writer.len, 2);
        let decoded: u32 = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, val);
    }

    // 3. 256 <= Value <= 65535 (3 bytes)
    for val in [256u32, 4096, 65535] {
        let writer = ConstCborWriter::<16>::new().write_u32(val);
        let bytes: [u8; 3] = extract_bytes(writer.buf);
        assert_eq!(writer.len, 3);
        let decoded: u32 = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, val);
    }

    // 4. Value > 65535 (5 bytes)
    for val in [65536u32, 1000000, 4294967295] {
        let writer = ConstCborWriter::<16>::new().write_u32(val);
        let bytes: [u8; 5] = extract_bytes(writer.buf);
        assert_eq!(writer.len, 5);
        let decoded: u32 = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, val);
    }
}

#[test]
fn test_cbor_write_str_lengths() {
    // 1. Empty string
    {
        let writer = ConstCborWriter::<16>::new().write_str("");
        let bytes: [u8; 1] = extract_bytes(writer.buf);
        assert_eq!(writer.len, 1);
        let decoded: &str = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, "");
    }

    // 2. Short string (len <= 23)
    {
        let s = "rp2040";
        let writer = ConstCborWriter::<16>::new().write_str(s);
        let bytes: [u8; 7] = extract_bytes(writer.buf);
        assert_eq!(writer.len, 7);
        let decoded: &str = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, s);
    }

    // 3. Medium string (24 <= len <= 255)
    {
        let s = "A".repeat(100);
        let writer = ConstCborWriter::<128>::new().write_str(&s);
        assert_eq!(writer.len, 102); // 1 tag + 1 len byte + 100 data bytes

        // Dynamic extraction for decoding
        let mut bytes = vec![0u8; writer.len];
        bytes.copy_from_slice(&writer.buf[..writer.len]);
        let decoded: &str = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, s);
    }

    // 4. Long string (len > 255)
    {
        let s = "B".repeat(300);
        let writer = ConstCborWriter::<512>::new().write_str(&s);
        assert_eq!(writer.len, 303); // 1 tag + 2 len bytes + 300 data bytes

        let mut bytes = vec![0u8; writer.len];
        bytes.copy_from_slice(&writer.buf[..writer.len]);
        let decoded: &str = minicbor::decode(&bytes).unwrap();
        assert_eq!(decoded, s);
    }
}

#[test]
fn test_cbor_write_map_header() {
    // Map with 5 elements
    let writer = ConstCborWriter::<16>::new().write_map_header(5);
    assert_eq!(writer.len, 1);
    assert_eq!(writer.buf[0], 0xa5);
}

#[test]
fn test_cbor_write_array_header() {
    // Array with 6 elements
    let writer = ConstCborWriter::<16>::new().write_array_header(6);
    assert_eq!(writer.len, 1);
    assert_eq!(writer.buf[0], 0x86);
}

#[test]
fn test_cbor_compound_structures() {
    // Encode a tuple representation: (42, "hello", 1000)
    let writer = ConstCborWriter::<32>::new()
        .write_array_header(3)
        .write_u32(42)
        .write_str("hello")
        .write_u32(1000);

    let mut bytes = vec![0u8; writer.len];
    bytes.copy_from_slice(&writer.buf[..writer.len]);

    let decoded: (u32, &str, u32) = minicbor::decode(&bytes).unwrap();
    assert_eq!(decoded, (42, "hello", 1000));
}
