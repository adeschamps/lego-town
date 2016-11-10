# LEGO Town

I collect [LEGO Modular Buildings](http://lego.wikia.com/wiki/Modular_Buildings) and I am (slowly) working on installing [addressable LEDs](https://learn.adafruit.com/adafruit-neopixel-uberguide/overview) in them.
This system consists of the following components:

* [client](./client/): The front end interface for communicating with the server
* [server](./server/): A server that keeps the clients in sync and tells the Arduino what to do
* An Arduino that controls the lights (not here yet)
* [mock-arduino](./mock-arduino/): A program that simulates the behaviour of the Arduino and renders "lights" as coloured circles
