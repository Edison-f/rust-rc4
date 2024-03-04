pub(crate) fn md5_runner() {
    let digest = md5::compute(vec![
        0xd1, 0x31, 0xdd, 0x02, 0xc5, 0xe6, 0xee, 0xc4, 0x69, 0x3d, 0x9a, 0x06, 0x98, 0xaf, 0xf9, 0x5c,
        0x2f, 0xca, 0xb5, 0x87, 0x12, 0x46, 0x7e, 0xab, 0x40, 0x04, 0x58, 0x3e, 0xb8, 0xfb, 0x7f, 0x89,
        0x55, 0xad, 0x34, 0x06, 0x09, 0xf4, 0xb3, 0x02, 0x83, 0xe4, 0x88, 0x83, 0x25, 0x71, 0x41, 0x5a,
        0x08, 0x51, 0x25, 0xe8, 0xf7, 0xcd, 0xc9, 0x9f, 0xd9, 0x1d, 0xbd, 0xf2, 0x80, 0x37, 0x3c, 0x5b,
        0x96, 0x0b, 0x1d, 0xd1, 0xdc, 0x41, 0x7b, 0x9c, 0xe4, 0xd8, 0x97, 0xf4, 0x5a, 0x65, 0x55, 0xd5,
        0x35, 0x73, 0x9a, 0xc7, 0xf0, 0xeb, 0xfd, 0x0c, 0x30, 0x29, 0xf1, 0x66, 0xd1, 0x09, 0xb1, 0x8f,
        0x75, 0x27, 0x7f, 0x79, 0x30, 0xd5, 0x5c, 0xeb, 0x22, 0xe8, 0xad, 0xba, 0x79, 0xcc, 0x15, 0x5c,
        0xed, 0x74, 0xcb, 0xdd, 0x5f, 0xc5, 0xd3, 0x6d, 0xb1, 0x9b, 0x0a, 0xd8, 0x35, 0xcc, 0xa7, 0xe3,
        ]);
    println!("{:?}", digest);
    let digest = md5::compute(vec![
        0xd1, 0x31, 0xdd, 0x02, 0xc5, 0xe6, 0xee, 0xc4, 0x69, 0x3d, 0x9a, 0x06, 0x98, 0xaf, 0xf9, 0x5c,
        0x2f, 0xca, 0xb5, 0x07, 0x12, 0x46, 0x7e, 0xab, 0x40, 0x04, 0x58, 0x3e, 0xb8, 0xfb, 0x7f, 0x89,
        0x55, 0xad, 0x34, 0x06, 0x09, 0xf4, 0xb3, 0x02, 0x83, 0xe4, 0x88, 0x83, 0x25, 0xf1, 0x41, 0x5a,
        0x08, 0x51, 0x25, 0xe8, 0xf7, 0xcd, 0xc9, 0x9f, 0xd9, 0x1d, 0xbd, 0x72, 0x80, 0x37, 0x3c, 0x5b,
        0x96, 0x0b, 0x1d, 0xd1, 0xdc, 0x41, 0x7b, 0x9c, 0xe4, 0xd8, 0x97, 0xf4, 0x5a, 0x65, 0x55, 0xd5,
        0x35, 0x73, 0x9a, 0x47, 0xf0, 0xeb, 0xfd, 0x0c, 0x30, 0x29, 0xf1, 0x66, 0xd1, 0x09, 0xb1, 0x8f,
        0x75, 0x27, 0x7f, 0x79, 0x30, 0xd5, 0x5c, 0xeb, 0x22, 0xe8, 0xad, 0xba, 0x79, 0x4c, 0x15, 0x5c,
        0xed, 0x74, 0xcb, 0xdd, 0x5f, 0xc5, 0xd3, 0x6d, 0xb1, 0x9b, 0x0a, 0x58, 0x35, 0xcc, 0xa7, 0xe3,
    ]);
    println!("{:?}", digest)
}