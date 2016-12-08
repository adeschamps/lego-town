
To cross compile for the Arduino, you need a CMake toolchain.
I have been using [this one](http://github.com/altexdim/arduino-cmake)

At the moment, external dependencies are not properly set up, so the build is a little clunky:

```bash
$ mkdir build
$ cd build
$ cmake -DCMAKE_TOOLCHAIN_FILE=<path-to-toolchain> ..
$ make # expect to fail
$ cmake .
$ make
```

Additionally, there are a few hard coded paths in the CMakeLists.txt file.