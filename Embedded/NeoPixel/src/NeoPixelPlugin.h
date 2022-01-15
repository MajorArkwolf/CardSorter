#pragma once
#include "Arduino.h"
#include <Adafruit_NeoPixel.h>

class NeoPixelPlugin {
  public:
  NeoPixelPlugin();
  NeoPixelPlugin(int ledPin, int stripLength);
  void ParseMessage(const String& message);
  
  private:
  Adafruit_NeoPixel m_strip;
  int m_stripLength;
};
