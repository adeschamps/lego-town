#include "light_controller.hpp"

// Drive the main logic of the Arduino.
int main()
{
  Poco::Net::SocketAddress address (Poco::Net::IPAddress(), 12345);
  LightController arduino (address);
  arduino.run();
}
