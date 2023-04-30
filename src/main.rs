use minecraft_world::NbtValue;

fn main() {
    let mut args = std::env::args();
    let file_name = args.nth(1).expect("failed to provide file");
    let out_file_name = args.next().unwrap_or(String::from("/tmp/nbt"));
    println!("file_name: {file_name}, out_file_name: {out_file_name}");
    let mut file = std::fs::File::open(file_name).expect("failed to read file");
    let (name, nbt) = NbtValue::from_compressed_reader(file).expect("failed to parse file");
    eprintln!("{nbt:?}");
    let out_file = std::fs::File::create(out_file_name).expect("failed to open file");
    nbt.to_compressed_writer(&name, out_file)
        .expect("failed to write file");
}
