#pragma once
#include "../../Color.hpp"
#include <stdint.h>

namespace IO {
    using SensorID = uint8_t;

    namespace Definition {
        enum class SensorType {
            None,
            PixelLight,
            PhotoResistor,
            ServoMotor,
            DeattachedServoMotor,
        };
    }
    struct NSensor {
        int BoardID;
        NSensor() : BoardID(0) {}
        NSensor(int boardID) : BoardID(boardID) {}
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
        ServoMotorData(int pin) : Pin(pin) {}
    };

    enum class ServoMotorMethods {
        Get,
        Set,
    };
    
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

    union SensorInitData
    {
        NSensor networkSensor;
        PhotoResitorData photoResitorData;
        ServoMotorData servoMotorData;
        PixelLightData pixelLightData;
        SensorInitData();
        SensorInitData(NSensor data);
        SensorInitData(PhotoResitorData data);
        SensorInitData(ServoMotorData data);
        SensorInitData(PixelLightData data);

    };
}