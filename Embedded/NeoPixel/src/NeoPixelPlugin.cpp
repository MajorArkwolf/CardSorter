#include "NeoPixelPlugin.h"

template<class T>
const T& clamp(const T& x, const T& upper, const T& lower) {
    return min(upper, max(x, lower));
}

NeoPixelPlugin::NeoPixelPlugin() : m_strip(24, 9, NEO_GRB + NEO_KHZ800) {
  m_strip.begin();
  m_strip.show();
}

NeoPixelPlugin::NeoPixelPlugin(int ledPin, int stripLength) : m_stripLength(stripLength), m_strip(stripLength, ledPin, NEO_GRB + NEO_KHZ800) {
  m_strip.begin();
  m_strip.show();
  uint32_t defaultColor = m_strip.Color(255, 255, 255);
  m_strip.fill(defaultColor, 0, m_stripLength);
  m_strip.show();
}

void NeoPixelPlugin::ParseMessage(const String& message) {
    // Red, Green, Blue and the Position to set.
    // Position of -1 is all pixels
    int16_t rbgp[] = { 0, 0, 0, -1 };
    unsigned index = 0;

    // Range from start of value to end of value
    int start_value = 0;
    int end_value = 0;

    while (true) {
        end_value = message.indexOf(' ', start_value);
        // If the next value is not found we break
        if (end_value < 0) {
          break;
        }

        // Start value is inclusive in the copy, but the end value is excluded.
        rbgp[index] = message.substring(start_value, end_value).toInt();
        ++index;

        // Stop when we have populated the max array values
        if (index >= 4) {
          break;
        }

        // Since the current end value is a whitespace we skip 1 ahead of it
        start_value = end_value + 1;

        // Check to ensure that we dont overshoot the length
        if (start_value >= message.length()) {
          break;
        }
    }

    // We can have an RGB or RGBP but nothing less, so dont set anything
    // if we didnt get atleast 3 values.
    if (index < 3) {
      return;
    }

    uint32_t color = m_strip.Color((uint8_t)clamp(rbgp[0], 0, 255), (uint8_t)clamp(rbgp[1], 0, 255), (uint8_t)clamp(rbgp[2], 0, 255));

    // A position of -1 translates to a fill, so we set all pixels to this color, else we set the individual color.
    if (rbgp[3] == -1) {
      m_strip.fill(color, 0, m_stripLength);
    } else {
      uint16_t position = clamp(rbgp[3], 0, m_stripLength);
      m_strip.setPixelColor(position, color);
    }
    // Update NeoPixel to show changes
    m_strip.show();
}
