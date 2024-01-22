export def "to nbt" [] {
  to json | minecraft_world from-json
}
export def "from nbt" [] {
  minecraft_world to-json | from json
}
export def "from c-nbt" [] {
  minecraft_world compressed-to-json | from json
}
export def "to c-nbt" [] {
  to json | minecraft_world compressed-from-json
}
