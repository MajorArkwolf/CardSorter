#pragma once
#include "BoardDefinitions.hpp"

namespace Shared {
    struct Color {
        uint8_t red;
        uint8_t green;
        uint8_t blue;
        Color();
        Color(uint8_t r, uint8_t g, uint8_t b);
    };
}