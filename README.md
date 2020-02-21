## Chip8 Emulator using Rust
[![Build Status](https://travis-ci.org/asnimansari/chip8.svg?branch=master)](https://travis-ci.org/asnimansari/chip8)

Created this basic chip8 emulator to learn more about emulator programming as well as Rust language.
There are pending features to be implemented like `Sound` which will be added soon.

Games are included in `data` folder.

### Screenshot

![](images/space-invaders.png)


### Keyboard Map
![](images/keyboard-map.png)


### Build
For production build 
```
cargo build --release
```

and give the same path as argument like.. 

```
./chip-8-rust programs/INVADERS 
```

For developers please use  `cargo run` inside the project directory to start