# void
hello there

# install to nushell
install minecraft_world binary:
```nu
cargo install --git https://github.com/EsfoNL/minecraft_world_rs.git
```
add to nbt.nu to scripts path. \
add to nu config:
```nu
source nbt.nu
```
## usage
```
from nbt
```
convert uncompressed nbt to structured data
```
from c-nbt
```
convert compressed nbt to structured data
```
to nbt
```
convert structured data to uncompressed nbt
```
to c-nbt
```
convert structured data to compressed nbt

nbt format specs taken from [NBT format](https://minecraft.wiki/w/NBT_format#Binary_format)
