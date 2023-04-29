use minecraft_world::NbtValue;

fn main() {
    let file_name = std::env::args().nth(1).expect("failed to provide file");
    println!("file_name: {file_name}");
    let file = std::fs::read(file_name).expect("failed to read file");
    let (name, nbt) = NbtValue::from_bytes(&file).expect("failed to parse nbt");
    println!("\"{name}\": {:?}", nbt);
}
