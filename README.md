# Chip-8 Emulator

## Controls

The controls are mapped to the following keys:

```
1 2 3 4
q w e r
a s d f
z x c v
```

## Installation Instructions

There are two options for running the emulator: SDL (gui) or WASM (in the browser).

### SDL

First, clone this repo to your computer.

Then navigate to the `desktop` folder.

```console
cd desktop/
```

Next, run:

```console
cargo run <path_to_chip-8_rom>
```

### WASM

First, clone this repo to your computer.

Then navigate to the `web` folder.

```console
cd web/
```

Next, run a web server using your preferred method.
For example:

```console
python3 -m http.server
```

Finally, browse to the website's address that is being served by the web server in your browser.
Click `Browse...` to upload a Chip-8 ROM from your computer.

Have fun!

## Credits

A big thanks to the following resources:

- https://aquova.net/blog/chip8/
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
- https://austinmorlan.com/posts/chip8_emulator/
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

