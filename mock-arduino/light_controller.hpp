#ifndef LIGHT_CONTROLLER_HPP
#define LIGHT_CONTROLLER_HPP

#include "arduino.hpp"
#include "messages.pb.h"

#include <Poco/Net/DatagramSocket.h>
#include <SFML/Graphics.hpp>
#include <vector>

// This class listens for commands from the server
// and sets the colours of "lights" accordingly.
// It renders the lights as rows of coloured circles.
class LightController : public Arduino
{
public:
  LightController(Poco::Net::SocketAddress address, std::vector<size_t> const & lengths);

private:
  virtual LoopStatus loop() override;

  // Handle commands from the server
  void handle_messages();
  bool handle_message(light_controller::SetLights const &);

  // Draws the window
  void draw();

  // Represents the state of a light.
  struct Light {
    Light() = default;
    Light(sf::Color color) : color(color) {}
    sf::Color color;
  };

  // The actual Arduino has multiple chains of lights controlled by different pins
  using lightstrip_t = std::vector<Light>;
  std::vector<lightstrip_t> lightstrips;

  // Display
  int radius = 20;
  sf::RenderWindow window;

  // Messaging
  Poco::Net::SocketAddress address;
  Poco::Net::DatagramSocket incoming_socket;
  std::string message_buffer;
};

#endif
