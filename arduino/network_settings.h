// #define SSID "your network"
// #define PASSWORD "your password"
// #define INCOMING_MESSAGES_PORT 12345

#if !defined SSID || !defined PASSWORD || !defined INCOMING_MESSAGES_PORT
#define ERROR_MSG "Please #define network settings in network_settings.h"
#endif
