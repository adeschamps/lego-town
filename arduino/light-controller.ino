#include <Adafruit_NeoPixel.h>
// #include <ESP8266WiFi.h>
#include <SoftwareSerial.h>
#include <SparkFunESP8266WiFi.h>
#include <SPI.h>
#include <pb_decode.h>

#include "log.hpp"

#define NETWORK_SSID "Deschamps"
#define NETWORK_PASSWORD "31803180"

#define NUM_LIGHTS 100
Adafruit_NeoPixel lights = Adafruit_NeoPixel(NUM_LIGHTS, 6, NEO_RGB + NEO_KHZ800);

void setup()
{
  if (! esp8266.begin())
    LOG_FATAL("Failed to initialize ESP8266");

  switch (esp8266.connect(NETWORK_SSID, NETWORK_PASSWORD))
  {
  case ESP8266_RSP_SUCCESS:
    break;

  case ESP8266_CMD_BAD:
  case ESP8266_RSP_MEMORY_ERR:
  case ESP8266_RSP_FAIL:
  case ESP8266_RSP_UNKNOWN:
  case ESP8266_RSP_TIMEOUT:
  default:
    LOG_FATAL("Failed to connect");
  }
}

void loop()
{

}
