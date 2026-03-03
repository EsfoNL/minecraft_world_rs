use clap::Parser;
use minecraft_world::NbtFile;
#[derive(Parser)]
enum Options {
    FromJson,
    ToJson,
    CompressedFromJson,
    CompressedToJson,
    FromMsgPack,
    ToMsgPack,
    CompressedFromMsgPack,
    CompressedToMsgPack,
}

fn main() {
    let cmd = Options::parse();
    match cmd {
        Options::FromJson => serde_json::de::from_reader::<_, NbtFile>(std::io::stdin())
            .unwrap()
            .to_writer(std::io::stdout())
            .unwrap(),
        Options::ToJson => serde_json::ser::to_writer(
            std::io::stdout(),
            &NbtFile::from_reader(std::io::stdin()).unwrap(),
        )
        .unwrap(),
        Options::CompressedFromJson => serde_json::de::from_reader::<_, NbtFile>(std::io::stdin())
            .unwrap()
            .to_compressed_writer(std::io::stdout())
            .unwrap(),
        Options::CompressedToJson => serde_json::ser::to_writer(
            std::io::stdout(),
            &NbtFile::from_compressed_reader(std::io::stdin()).unwrap(),
        )
        .unwrap(),
        Options::FromMsgPack => rmp_serde::decode::from_read::<_, NbtFile>(std::io::stdin())
            .unwrap()
            .to_writer(std::io::stdout())
            .unwrap(),
        Options::ToMsgPack => rmp_serde::encode::write(
            &mut std::io::stdout(),
            &NbtFile::from_reader(std::io::stdin()).unwrap(),
        )
        .unwrap(),
        Options::CompressedFromMsgPack => {
            rmp_serde::decode::from_read::<_, NbtFile>(std::io::stdin())
                .unwrap()
                .to_compressed_writer(std::io::stdout())
                .unwrap()
        }
        Options::CompressedToMsgPack => rmp_serde::encode::write(
            &mut std::io::stdout(),
            &NbtFile::from_compressed_reader(std::io::stdin()).unwrap(),
        )
        .unwrap(),
    }
}
