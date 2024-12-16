pub fn main() {
    let data = "Hello";
    let mut encoder = ideen_qr::Encoder::new(ideen_qr::Format::auto(data));
    let buffer = encoder.encode_str(data);
    buffer.to_image(4).save("out.png").unwrap();
}
