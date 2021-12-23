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
            Motor,
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

    struct MotorData {
        int Pin;
        MotorData(int pin) : Pin(pin) {}
    };

    enum class ServoMotorMethods {
        Get,
        Set,
    };
    
    enum class MotorMethods {
        Get,
        Set,
        SetOnTimer,
    };

    union SensorDataTypes
    {
        bool boolean;
        int integar;
        Shared::Color color;
        SensorDataTypes();
        SensorDataTypes(const Shared::Color& newColor);
        SensorDataTypes(bool newBoolean);
        SensorDataTypes(int newIntegar);
    };

    union SensorInitData
    {
        NSensor networkSensor;
        PhotoResitorData photoResitorData;
        MotorData motorData;
        PixelLightData pixelLightData;
        SensorInitData();
        SensorInitData(NSensor data);
        SensorInitData(PhotoResitorData data);
        SensorInitData(MotorData data);
        SensorInitData(PixelLightData data);

    };
}