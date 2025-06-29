# Quick start

## Playing songs with lyrics

You need to put music and lyrics in the `music` subdirectory.

Each song should have the same base stem, e.g. `boop.mp3` and `boop.lrc`.

You can then play the song with:

```
cargo run play music/boop.mp3
```

### Debugging

Note that if the program crashes, you may not see a backtrace, or even
any hint of what caused the crash. If this happens, try the following:

```
cargo run play music/boop.mp3 2> stderr.log
```

Then open `stderr.log` after the program crashes.

## Decoding hangul

You can also decode individual strings of Hangul like this:

```
cargo run decode '밥을'
```
