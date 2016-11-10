cmake_minimum_required(VERSION 3.5)
project(lego-town)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)

# Mock Arduino
add_executable(mock-arduino
  main.cpp
  light_controller.cpp
)

# Generate protobuf messages and create a library
find_package(Protobuf REQUIRED)
include_directories(${PROTOBUF_INCLUDE_DIRS})
include_directories(${CMAKE_CURRENT_BINARY_DIR})
protobuf_generate_cpp(PROTO_SRCS PROTO_HDRS ../arduino-api/messages.proto)
add_library(messages ${PROTO_SRCS})
target_link_libraries(messages ${PROTOBUF_LIBRARIES})

# Link protobuf messages
target_link_libraries(mock-arduino messages)

# Find and link Simple Fast Multimedia Library
find_package(SFML COMPONENTS graphics system window REQUIRED)
include_directories(${SFML_INCLUDE_DIR})
target_link_libraries(mock-arduino ${SFML_LIBRARIES})

# Find and link Poco

find_package(Poco REQUIRED Net)
include_directories(${Poco_INCLUDE_DIRS})
target_link_libraries(mock-arduino ${Poco_LIBRARIES})
