#include "light_controller.hpp"

#include <sstream>

// Drive the main logic of the Arduino.
int main(int argc, char* argv[])
{
  // Optionally parse the port from the command line
  auto port = [&]{
    uint16_t result = 12345;
    if (argc > 1) {
      auto ss = std::stringstream(argv[1]);
      uint16_t port;
      if (ss >> port) { result = port; }
    }
    return result;
  }();

  Poco::Net::SocketAddress address (port);
  LightController arduino (address);
  arduino.run();
}
