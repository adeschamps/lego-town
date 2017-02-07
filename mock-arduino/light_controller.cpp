#include "light_controller.hpp"

#include <chrono>
#include <iostream>
#include <thread>

LightController::LightController(Poco::Net::SocketAddress address, std::vector<size_t> const & lengths)
  : window(sf::VideoMode(100, 100), "LEGO Town")
  , address(address)
  , message_buffer(100, '\0')
{
  incoming_socket.bind(address);
  incoming_socket.setBlocking(false);


  lightstrips.clear();
  for (auto length : lengths)
    lightstrips.emplace_back(length);
}

Arduino::LoopStatus LightController::loop()
{
  using namespace std::chrono_literals;
  std::this_thread::sleep_for(10ms);

  if (window.isOpen() == false)
    return DONE;

  sf::Event event;
  while(window.pollEvent(event))
  {
    if (event.type == sf::Event::Closed
        || sf::Keyboard::isKeyPressed(sf::Keyboard::Q)
        || sf::Keyboard::isKeyPressed(sf::Keyboard::Escape))
      window.close();
  }

  handle_messages();
  draw();

  return CONTINUE;
}


void LightController::handle_messages()
{
  int bytes = incoming_socket.receiveBytes((void*)message_buffer.c_str(), message_buffer.size());
  if (bytes < 0) return;

  std::cout << "Received " << bytes << " bytes: ";
  for (int i = 0; i != bytes; ++i)
    printf("%02x ", (unsigned char)message_buffer[i]);
  std::cout << '\n';

  light_controller::Command command;
  if( ! command.ParseFromArray(message_buffer.c_str(), bytes))
  { std::cerr << "failed to parse\n"; return; }
  std::cerr << command.DebugString();

  bool success = true;

  using CT = light_controller::Command::CommandTypeCase;
  success &= [&]{switch(command.CommandType_case()) {
    case CT::kSetLights: return handle_message(command.set_lights());
    default: return false;
    }}();

  if (!success){ std::cerr << "failed to execute command\n"; return; }
}


sf::Color parseColor(light_controller::Color const & color)
{
  using namespace light_controller;
  switch (color)
  {
  case OFF:     return sf::Color(  0,   0,   0);
  case WHITE:   return sf::Color(255, 255, 255);
  case RED:     return sf::Color(255,   0,   0);
  case ORANGE:  return sf::Color(255, 128,   0);
  case YELLOW:  return sf::Color(255, 255,   0);
  case GREEN:   return sf::Color(  0, 255,   0);
  case CYAN:    return sf::Color(  0, 255, 255);
  case BLUE:    return sf::Color(  0,   0, 255);
  case PURPLE:  return sf::Color(128,   0, 255);
  case MAGENTA: return sf::Color(255,   0, 255);
  default: return sf::Color(100, 100, 100);
  }
}


bool LightController::handle_message(light_controller::SetLights const & set_lights)
{
  auto light_group = set_lights.light_group();
  auto light_id_start = set_lights.light_id_start();
  auto light_id_end = set_lights.light_id_end();
  auto color = parseColor(set_lights.color());

  if (light_group >= lightstrips.size()) return false;
  auto & lightstrip = lightstrips[light_group];

  std::cerr << "valid light group\n";
  std::cerr << "lightstrip.size() :: " << lightstrip.size() << '\n';

  if (light_id_end > lightstrip.size()) return false;

  std::cerr << "valid light end\n";

  if (light_id_end < light_id_start) return false;

  std::cerr << "valid light start\n";

  for (auto light_id = light_id_start; light_id != light_id_end; ++light_id)
    lightstrip[light_id].color = color;

  return true;
}


void LightController::draw()
{
  window.clear();

  for (uint row = 0; row != lightstrips.size(); ++row)
  {
    auto const & lightstrip = lightstrips[row];
    for (uint col = 0; col != lightstrip.size(); ++col)
    {
      auto const & light = lightstrip[col];
      auto shape = sf::CircleShape(radius);
      auto color = light.color;
      shape.setFillColor(color);
      shape.setOutlineColor(sf::Color::White);
      shape.setOutlineThickness(2.0f);

      auto x = col * 2.5 * radius + 5;
      auto y = row * 3.0 * radius + 5;
      shape.setPosition(x, y);
      window.draw(shape);
    }
  }

  window.display();
}
