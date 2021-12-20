#pragma once

#ifndef _COMPUTER_
  #define _ARDUINO_ 1
#endif

#ifdef _COMPUTER_
  #include <stdint.h>
  #include <string.h>
  #ifndef byte
    #define byte uint8_t
  #endif
#endif

#ifdef _ARDUINO_
  #include <Arduino.h>
#endif

// Comms Logic
#define DRIVER_BOARD 1
#define PASSENGER_BOARD 2

// Board Addresses
#define BOARD_1 1
#define BOARD_2 2
#define BOARD_3 3