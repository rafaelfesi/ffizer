fn main() {
    // let v: [u8; 22] = [
    //     73, 39, 109, 32, 102, 105, 108, 101, 95, 53, 95, 109, 121, 45, 112, 114, 111, 106, 101,
    //     116, 46, 10,
    // ];
    let v: [u8; 13] = [73, 39, 109, 32, 102, 105, 108, 101, 95, 53, 95, 46, 10];
    eprintln!("{}", std::str::from_utf8(&v).unwrap());
}
