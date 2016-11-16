#include "light_controller.hpp"

#include <chrono>
#include <iostream>
#include <thread>

LightController::LightController(Poco::Net::SocketAddress address)
  : window(sf::VideoMode(100, 100), "LEGO Town")
  , address(address)
  , message_buffer(100, '\0')
{
  for (auto length : {3, 3, 2, 3})
    lightstrips.emplace_back(length);

  incoming_socket.bind(address);
  incoming_socket.setBlocking(false);
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
    case CT::kSetLight: return handle_message(command.set_light());
    case CT::kSetGroup: return handle_message(command.set_group());
    default: return false;
    }}();

  if (!success){ std::cerr << "failed to execute command\n"; return; }
}


sf::Color parseColor(light_controller::Color const & color)
{
  return sf::Color(color.red(), color.green(), color.blue());
}


bool LightController::handle_message(light_controller::SetLight const & set_light)
{

  auto light_group = set_light.light_group();
  auto lightId = set_light.light_id();
  auto color = parseColor(set_light.color());

  if (light_group >= lightstrips.size()) return false;
  auto & lightstrip = lightstrips[light_group];
  if (lightId >= lightstrip.size()) return false;
  auto & light = lightstrip[lightId];
  light.color = color;

  return true;
}


bool LightController::handle_message(light_controller::SetGroup const & set_group)
{
  auto light_group = set_group.light_group();
  auto color = parseColor(set_group.color());

  if (light_group >= lightstrips.size()) return false;
  auto & lightstrip = lightstrips[light_group];
  for (auto & light : lightstrip)
    light.color = color;
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
