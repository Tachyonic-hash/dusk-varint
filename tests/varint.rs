extern crate alloc;

use dusk_varint::VarInt;

#[test]
fn test_required_space() {
    assert_eq!((0 as u32).required_space(), 1);
    assert_eq!((1 as u32).required_space(), 1);
    assert_eq!((128 as u32).required_space(), 2);
    assert_eq!((16384 as u32).required_space(), 3);
    assert_eq!((2097151 as u32).required_space(), 3);
    assert_eq!((2097152 as u32).required_space(), 4);
}

#[test]
fn test_identity_u64() {
    let mut buffer = [0u8; 128];
    for i in 1 as u64..100 {
        i.encode_var(&mut buffer);
        assert_eq!(u64::decode_var(&buffer).unwrap(), (i, 1));
    }
    for i in 16400 as u64..16500 {
        i.encode_var(&mut buffer);
        assert_eq!(u64::decode_var(&buffer).unwrap(), (i, 3));
    }
}

#[test]
fn test_decode_max_u64() {
    let max_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
    assert_eq!(
        u64::decode_var(max_vec_encoded.as_slice()).unwrap().0,
        u64::max_value()
    );
}

#[test]
fn test_encode_i64() {
    let mut buffer_a = [0u8; 128];
    let mut buffer_b = [0u8; 128];

    (0 as i64).encode_var(&mut buffer_a);
    (0 as u32).encode_var(&mut buffer_b);
    assert_eq!(buffer_a, buffer_b);

    (150 as i64).encode_var(&mut buffer_a);
    (300 as u32).encode_var(&mut buffer_b);
    assert_eq!(buffer_a, buffer_b);

    (-150 as i64).encode_var(&mut buffer_a);
    (299 as u32).encode_var(&mut buffer_b);
    assert_eq!(buffer_a, buffer_b);

    (-2147483648 as i64).encode_var(&mut buffer_a);
    (4294967295 as u64).encode_var(&mut buffer_b);
    assert_eq!(buffer_a, buffer_b);

    (i64::max_value() as i64).encode_var(&mut buffer_a);
    assert_eq!(
        &buffer_a[0..10],
        &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
    );
    (i64::min_value() as i64).encode_var(&mut buffer_a);
    assert_eq!(
        &buffer_a[0..10],
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
    );
}

#[test]
fn test_decode_min_i64() {
    let min_vec_encoded = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
    assert_eq!(
        i64::decode_var(min_vec_encoded).unwrap().0,
        i64::min_value()
    );
}

#[test]
fn test_decode_max_i64() {
    let max_vec_encoded = &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
    assert_eq!(
        i64::decode_var(max_vec_encoded).unwrap().0,
        i64::max_value()
    );
}

#[test]
fn test_encode_i16() {
    let mut buffer_a = [0u8; 128];
    let mut buffer_b = [0u8; 128];

    (150 as i16).encode_var(&mut buffer_a);
    (300 as u32).encode_var(&mut buffer_b);

    assert_eq!(buffer_a, buffer_b);

    (-150 as i16).encode_var(&mut buffer_a);
    (299 as u32).encode_var(&mut buffer_b);

    assert_eq!(buffer_a, buffer_b);
}
