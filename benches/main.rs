use bencher::Bencher;

use dusk_varint::*;

fn encode_v(b: &mut Bencher) {
    let my_u64s = [
        9494929199119074561,
        3823923198123425321,
        2595862268225688522,
        1231230009321245673,
        2909812083312547546,
        3492011001874124465,
        4848848884210156473,
        4012941340125654654,
    ] as [u64; 8];
    let my_i64s = [
        -122193043711204545,
        2446312246543212452,
        -445854216865433664,
        3242135654513135465,
        -543122132545464312,
        3613543123031434343,
        -431231254654543211,
        7854615463131234543,
    ] as [i64; 8];

    let mut dst = [0 as u8; 10];

    b.iter(|| {
        // 8x each.
        my_u64s[0].encode_var(&mut dst);
        my_u64s[1].encode_var(&mut dst);
        my_u64s[2].encode_var(&mut dst);
        my_u64s[3].encode_var(&mut dst);
        my_u64s[4].encode_var(&mut dst);
        my_u64s[5].encode_var(&mut dst);
        my_u64s[6].encode_var(&mut dst);
        my_u64s[7].encode_var(&mut dst);

        my_i64s[0].encode_var(&mut dst);
        my_i64s[1].encode_var(&mut dst);
        my_i64s[2].encode_var(&mut dst);
        my_i64s[3].encode_var(&mut dst);
        my_i64s[4].encode_var(&mut dst);
        my_i64s[5].encode_var(&mut dst);
        my_i64s[6].encode_var(&mut dst);
        my_i64s[7].encode_var(&mut dst);
    });
}

fn decode_v(b: &mut Bencher) {
    let my_u64s = [
        9494929199119074561,
        3823923198123425321,
        2595862268225688522,
        1231230009321245673,
        2909812083312547546,
        3492011001874124465,
        4848848884210156473,
        4012941340125654654,
    ] as [u64; 8];
    let my_i64s = [
        -122193043711204545,
        2446312246543212452,
        -445854216865433664,
        3242135654513135465,
        -543122132545464312,
        3613543123031434343,
        -431231254654543211,
        7854615463131234543,
    ] as [i64; 8];

    let mut buffers_u64 = [[0u8; 16]; 8];
    let mut buffers_i64 = [[0u8; 16]; 8];

    my_u64s[0].encode_var(&mut buffers_u64[0]);
    my_u64s[1].encode_var(&mut buffers_u64[1]);
    my_u64s[2].encode_var(&mut buffers_u64[2]);
    my_u64s[3].encode_var(&mut buffers_u64[3]);
    my_u64s[4].encode_var(&mut buffers_u64[4]);
    my_u64s[5].encode_var(&mut buffers_u64[5]);
    my_u64s[6].encode_var(&mut buffers_u64[6]);
    my_u64s[7].encode_var(&mut buffers_u64[7]);

    my_i64s[0].encode_var(&mut buffers_i64[0]);
    my_i64s[1].encode_var(&mut buffers_i64[1]);
    my_i64s[2].encode_var(&mut buffers_i64[2]);
    my_i64s[3].encode_var(&mut buffers_i64[3]);
    my_i64s[4].encode_var(&mut buffers_i64[4]);
    my_i64s[5].encode_var(&mut buffers_i64[5]);
    my_i64s[6].encode_var(&mut buffers_i64[6]);
    my_i64s[7].encode_var(&mut buffers_i64[7]);

    b.iter(|| {
        // 8x each.
        u64::decode_var(&buffers_u64[0]).unwrap();
        u64::decode_var(&buffers_u64[1]).unwrap();
        u64::decode_var(&buffers_u64[2]).unwrap();
        u64::decode_var(&buffers_u64[3]).unwrap();
        u64::decode_var(&buffers_u64[4]).unwrap();
        u64::decode_var(&buffers_u64[5]).unwrap();
        u64::decode_var(&buffers_u64[6]).unwrap();
        u64::decode_var(&buffers_u64[7]).unwrap();

        i64::decode_var(&buffers_i64[0]).unwrap();
        i64::decode_var(&buffers_i64[1]).unwrap();
        i64::decode_var(&buffers_i64[2]).unwrap();
        i64::decode_var(&buffers_i64[3]).unwrap();
        i64::decode_var(&buffers_i64[4]).unwrap();
        i64::decode_var(&buffers_i64[5]).unwrap();
        i64::decode_var(&buffers_i64[6]).unwrap();
        i64::decode_var(&buffers_i64[7]).unwrap();
    });
}

bencher::benchmark_group!(varint_benches, encode_v, decode_v);

bencher::benchmark_main!(varint_benches);
