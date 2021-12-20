#pragma once
#include "../../BoardDefinitions.hpp"
#include "SensorDefinitions.hpp"

namespace IO {
    
    union Method {
        ServoMotorMethods servoMotorMethod;
        PixelLightMethods pixelLightMethod;
        PhotoResistorMethods photoResistorMethod;
        Method();
    };

    struct SensorMessage {
        int sensorID;
        Definition::SensorType type;
        Method method;
        SensorDataTypes data;
        SensorMessage();
    };

    struct SensorMessageResponse {
        bool wasSuccessful;
        SensorDataTypes data;
        SensorMessageResponse();
    };
}