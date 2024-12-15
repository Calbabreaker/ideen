pub fn main() {
    let data = "Hello";
    let mut encoder = ideen_qr::Encoder::new(ideen_qr::Format::auto(data));
    let image = encoder.encode_str(data);
    image.save("out.png").unwrap();
}
