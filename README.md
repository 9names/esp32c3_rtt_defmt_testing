# esp32c3_rtt_defmt_testing
A very rough demo that's just enough to get defmt running.

## Direct-boot mode

in Embed.toml
```
[default.flashing]
enabled = true
```

then

```system
cargo embed --release --features directboot
```

## Bootloader mode
in Embed.toml
```
[default.flashing]
enabled = false
```

then

```system
cargo espflash --release && cargo embed --release
```

you'll get some gibberish at the start, but it *should* recover
