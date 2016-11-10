# Mock Arduino

This program simulates the behaviour of the Arduino.
It listens for the same protobuf messages over UDP,
and its code is loosely structured in the same way as the Arduino code will be.

Dependencies:

* [POCO](https://pocoproject.org) for networking.
* [SFML](http://www.sfml-dev.org) for rendering.
* [Protocol Buffers](https://developers.google.com/protocol-buffers) for message serialization.

Usage:

```bash
$ mkdir build
$ cd build
$ cmake ..
$ make && ./mock-arduino
```
