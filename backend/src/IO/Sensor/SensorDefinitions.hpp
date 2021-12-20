#pragma once
#include "../../Color.hpp"
#include <stdint.h>

namespace IO {
    namespace Definition {
        enum class SensorType {
            None,
            PixelLight,
            PhotoResistor,
            ServoMotor,
        };
    }

    union SensorDataTypes
    {
        bool boolean;
        int8_t integar;
        Shared::Color color;
        SensorDataTypes();
        SensorDataTypes(const Shared::Color& newColor);
        SensorDataTypes(bool newBoolean);
        SensorDataTypes(int8_t newIntegar);
    };
    
    struct PhotoResitorData {
        int Pin;
    };

    enum class PhotoResistorMethods {
        Get
    };

    struct PixelLightData {
        int Pin;
        int NumberOfPixels;
    };

    enum class PixelLightMethods {
        SetColor,
        Show
    };

    struct ServoMotorData {
        int Pin;
    };

    enum class ServoMotorMethods {
        Get,
        Set,
    };
}