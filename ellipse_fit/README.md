# ellipse-fit

Used for fitting ellipses on the images the vision pipeline recieves.

## Building
OpenCV negates all the niceness of Rust's build system so you'll have to do some things you'd normally do if you were working with a C++ project.

### OSX
- Make sure `libClang` and all of its associated libraries are in your library path and accessible by Rust.
  - Homebrew may have stashed them away in `/opt/` so you probably want to run a command similar to `cp /opt/homebrew/Cellar/llvm/13.0.0_2/lib/*.dylib /usr/local/lib/*.dylib` 
- Install OpenCV 4 (4! not 2!) with `brew install opencv` 
- `cargo build`

