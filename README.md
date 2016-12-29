## About

This is a small example of exporting a C string statically from a C-compatible shared library

The solution is based upon https://stackoverflow.com/questions/33850189/how-to-publish-a-constant-string-in-the-rust-ffi but I feel this is suboptimal.

Run `./build.sh` to build.

## Problems

* Cannot use `const fn` because the feature is unstable
* `*const c_char` does not implement `Sync` which is needed to have something static
* `&[u8].as_ptr()` is not a const fn so cannot be used in a static struct initializer
