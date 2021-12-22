#include "Color.hpp"

namespace Shared {
    Color::Color() : red(255), green(255), blue(255) {}
    Color::Color(uint8_t r, uint8_t g, uint8_t b) : red(r), green(g), blue(b) {}
}