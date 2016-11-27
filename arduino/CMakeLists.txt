cmake_minimum_required(VERSION 3.5)

set(ARDUINO_DEFAULT_BOARD uno)
set(ARDUINO_DEFAULT_PORT /dev/ttyACM0)
set(ARDUINO_DEFAULT_SERIAL picocom @SERIAL_PORT@ -b 9600 -l)

project(lego-town)

function(external_dependency target repo)
  include(ExternalProject)
  ExternalProject_Add(${target}
    GIT_REPOSITORY ${repo}
    UPDATE_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
  )
  ExternalProject_get_property(${target} source_dir)
  include_directories(${source_dir})
  include_directories(${source_dir}/src)
  link_directories(${source_dir})
  link_directories(${source_dir}/src)
endfunction()

external_dependency(esp8266 https://github.com/sparkfun/SparkFun_ESP8266_AT_Arduino_Library)
external_dependency(nanopb https://github.com/nanopb/nanopb)
external_dependency(neopixel https://github.com/adafruit/Adafruit_NeoPixel)

generate_arduino_firmware(light-controller
  SKETCH light-controller.ino
  PORT /dev/ttyACM0
)
add_dependencies(light-controller esp8266 nanopb neopixel)
