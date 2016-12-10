#include <Adafruit_NeoPixel.h>
#include <SoftwareSerial.h>
#include <WiFiEsp.h>
#include <WiFiEspUdp.h>
#include <messages.pb.h>
#include <pb_decode.h>

#include "log.hpp"

#include "network_settings.h"

#define NUM_LIGHTSTRIPS 3
uint8_t lightstrip_pins[NUM_LIGHTSTRIPS] = {6, 7, 8};
Adafruit_NeoPixel lightstrips[NUM_LIGHTSTRIPS];

#define BUFFER_SIZE light_controller_Command_size

WiFiEspUDP Udp;

uint32_t parse_color(light_controller_Color color);

#if 0
#define DEBUG_LIGHT(color) {                                            \
  for (uint8_t i = 0; i != NUM_LIGHTSTRIPS; ++i)                        \
  {                                                                     \
    lightstrips[i].setPixelColor(0, parse_color(light_controller_Color_##color)); \
    lightstrips[i].show();                                              \
  }                                                                     \
  }
#else
#define DEBUG_LIGHT(color) {}
#endif

#define DEBUG_BLINK(color) {                    \
  DEBUG_LIGHT(color);                           \
  delay(100);                                   \
  DEBUG_LIGHT(OFF);                             \
  delay(200);                                   \
  }

// Do nothing, forever.
inline void halt()
{
  while (true)
    delay(1000);
}

void setup()
{
  // Initialize lightstrips with 1 light
  for (uint8_t i = 0; i != NUM_LIGHTSTRIPS; ++i)
  {
    auto len = 9;
    auto pin = lightstrip_pins[i];
    lightstrips[i] = Adafruit_NeoPixel(len, pin, NEO_GRB + NEO_KHZ800);
    lightstrips[i].begin();
  }

  DEBUG_LIGHT(BLUE);

  // Initialize WiFi
  delay(500);
  Serial.begin(115200);
  WiFi.init(&Serial);
  delay(500);

  // Make sure WiFi shield is connected
  if (WiFi.status() == WL_NO_SHIELD)
  {
    DEBUG_LIGHT(RED);
    halt();
  }

  // Connect to network
  while (WiFi.begin(SSID, PASSWORD) != WL_CONNECTED)
  {
    DEBUG_BLINK(CYAN);
  }

  if (Udp.begin(INCOMING_MESSAGES_PORT) == 0)
  {
    DEBUG_LIGHT(MAGENTA);
    halt();
  }

  DEBUG_LIGHT(GREEN);
}

// Convert a color enum to an int for setting an LED
// This lives entirely in program memory; nothing is stored in data memory.
uint32_t parse_color(light_controller_Color color)
{
#define COLOR(color, r, g, b)                    \
  case light_controller_Color_##color:           \
    return Adafruit_NeoPixel::Color(r, g, b);

  switch (color)
  {
  COLOR(OFF,       0,   0,   0);
  COLOR(WHITE,   255, 255, 255);
  COLOR(RED,     255,   0,   0);
  COLOR(ORANGE,  255,  96,   0);
  COLOR(YELLOW,  255, 255,   0);
  COLOR(GREEN,     0, 255,   0);
  COLOR(CYAN,      0, 255, 255);
  COLOR(BLUE,      0,   0, 255);
  COLOR(PURPLE,  128,   0, 255);
  COLOR(MAGENTA, 255,   0, 255);
  }
#undef COLOR
}


inline void handle(light_controller_SetLights const & set_lights)
{
    auto & group_id = set_lights.light_group;
    auto & light_id_start = set_lights.light_id_start;
    auto & light_id_end = set_lights.light_id_end;
    auto & color = set_lights.color;

    if (group_id >= NUM_LIGHTSTRIPS) return;
    if (light_id_start > light_id_end) return;
    if (light_id_end > lightstrips[group_id].numPixels()) return;

    for (uint8_t i = light_id_start; i != light_id_end; ++i)
      lightstrips[group_id].setPixelColor(i, parse_color(color));
    lightstrips[group_id].show();
}


// NOTE: Currently a no-op
inline void handle(light_controller_Initialize const & initialize)
{
#if 0
  for (uint8_t i = 0; i != initialize.string_lengths_count; ++i)
  {
    DEBUG_BLINK(YELLOW);
  }

  // Reset
  for (uint8_t i = 0; i != NUM_LIGHTSTRIPS; ++i)
    lightstrips[i].updateLength(0);

  // Init
  for (uint8_t i = 0; i != initialize.string_lengths_count && i != NUM_LIGHTSTRIPS; ++i)
  {
    auto len = initialize.string_lengths[i];
    auto pin = lightstrip_pins[i];
    lightstrips[i] = Adafruit_NeoPixel(len, pin, NEO_GRB + NEO_KHZ800);
    lightstrips[i].begin();
  }
#endif
}

bool even_message = true;

inline void handle_messages()
{
  auto packet_length = Udp.parsePacket();
  if (packet_length == 0)
  {
    DEBUG_BLINK(WHITE);
    return;
  }

  DEBUG_BLINK(BLUE);

  uint8_t buffer[BUFFER_SIZE];
  memset(buffer, 0, BUFFER_SIZE);

  if (Udp.available() != packet_length)
  {
    DEBUG_LIGHT(ORANGE);
    halt();
  }

  auto len = Udp.read(buffer, BUFFER_SIZE);
  Udp.flush();
  if (len != packet_length)
  {
    DEBUG_BLINK(MAGENTA);
    return;
  }

  light_controller_Command command = light_controller_Command_init_zero;
  pb_istream_t stream = pb_istream_from_buffer(buffer, len);
  if (!pb_decode(&stream, light_controller_Command_fields, &command))
  {
    DEBUG_LIGHT(RED);
    return;
  }

  switch (command.which_CommandType)
  {
  case light_controller_Command_set_lights_tag:
    handle(command.CommandType.set_lights);
    break;

  case light_controller_Command_initialize_tag:
    handle(command.CommandType.initialize);
    break;
  }
}

// TODO: Implement various light transitions and effects
void update_lights()
{
  for (uint8_t i = 0; i != NUM_LIGHTSTRIPS; ++i)
  {
    lightstrips[i].show();
  }
}

void loop()
{
  handle_messages();
  update_lights();
}
