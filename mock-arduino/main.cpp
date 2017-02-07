#include "light_controller.hpp"
#include <tclap/CmdLine.h>

// Drive the main logic of the Arduino.
int main(int argc, char* argv[])
{
  TCLAP::CmdLine cmd("Mock Ardunio Light Controller");
  TCLAP::ValueArg<uint16_t> port_arg
    ("p", "port", "Incoming message port", false, 12345, "port", cmd);
  TCLAP::UnlabeledMultiArg<size_t> lengths_arg
    ("lenghts", "Number of lights in each strip", false, "lengths", cmd);
  cmd.parse(argc, argv);

  auto port = port_arg.getValue();

  Poco::Net::SocketAddress address (port);
  LightController arduino (address, lengths_arg.getValue());
  arduino.run();
}
