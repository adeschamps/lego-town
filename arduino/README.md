
To cross compile for the Arduino, you need a CMake toolchain.
I have been using [this one](http://github.com/altexdim/arduino-cmake)

```bash
$ mkdir build
$ cd build
$ cmake -DCMAKE_TOOLCHAIN_FILE=<path-to-toolchain> ..
$ make
```
