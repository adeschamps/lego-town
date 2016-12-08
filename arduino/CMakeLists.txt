cmake_minimum_required(VERSION 3.5)

set(ARDUINO_DEFAULT_BOARD uno)
set(ARDUINO_DEFAULT_PORT /dev/ttyACM0)
set(ARDUINO_DEFAULT_SERIAL picocom @SERIAL_PORT@ -b 9600 -l)

project(lego-town)
# Download an external library and include its headers and source files.
# This will fail on the first build, requiring `cmake .` to be run again.
# I'd like to fix that.
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

# A few modifications required for esp8266; see below.
# external_dependency(esp8266 https://github.com/bportaluri/WiFiEsp)
external_dependency(nanopb https://github.com/nanopb/nanopb)
external_dependency(neopixel https://github.com/adafruit/Adafruit_NeoPixel)

# This could be replaced by the above method if I fork the repo myself.
set(ESP_ROOT /home/anthony/git/WiFiEsp)
include_directories(${ESP_ROOT}/src)
set(ESP_SRCS
  ${ESP_ROOT}/src/WiFiEsp.cpp
  ${ESP_ROOT}/src/utility/EspDrv.cpp
  ${ESP_ROOT}/src/utility/RingBuffer.cpp
  ${ESP_ROOT}/src/WiFiEspUdp.cpp
)


# TODO: fix this up
set(CMAKE_MODULE_PATH /home/anthony/git/nanopb/extra)
find_package(Nanopb REQUIRED)
include_directories(${NANOPB_INCLUDE_DIRS})
message(STATUS "Protobuf executable: ${PROTOBUF_PROTOC_EXECUTABLE}")
message(STATUS "Nanopb generator source: ${NANOPB_GENERATOR_SOURCE_DIR}")

nanopb_generate_cpp(PROTO_SRCS PROTO_HDRS ${CMAKE_SOURCE_DIR}/../api/messages.proto)
# TODO: This feels somewhat dirty?
include_directories(${CMAKE_CURRENT_BINARY_DIR})
set_source_files_properties(${PROTO_SRCS} ${PROTO_HDRS}
    PROPERTIES GENERATED TRUE)


generate_arduino_firmware(light-controller
  SKETCH light-controller/light-controller.ino
  SRCS ${PROTO_SRCS} ${ESP_SRCS}
  PORT /dev/ttyACM0
)
add_dependencies(light-controller nanopb neopixel)
