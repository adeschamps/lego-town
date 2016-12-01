cmake_minimum_required(VERSION 3.5)

set(ARDUINO_DEFAULT_BOARD uno)
set(ARDUINO_DEFAULT_PORT /dev/ttyACM0)
set(ARDUINO_DEFAULT_SERIAL picocom @SERIAL_PORT@ -b 9600 -l)

project(lego-town)

# TODO: fix this up
set(CMAKE_MODULE_PATH /home/anthony/git/nanopb/extra)
find_package(Nanopb REQUIRED)
include_directories(${NANOPB_INCLUDE_DIRS})

nanopb_generate_cpp(PROTO_SRCS PROTO_HDRS ${CMAKE_SOURCE_DIR}/../api/messages.proto)
# TODO: This feels somewhat dirty?
include_directories(${CMAKE_CURRENT_BINARY_DIR})
set_source_files_properties(${PROTO_SRCS} ${PROTO_HDRS}
    PROPERTIES GENERATED TRUE)

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
  SRCS ${PROTO_SRCS}
  PORT /dev/ttyACM0
)
add_dependencies(light-controller esp8266 nanopb neopixel)
