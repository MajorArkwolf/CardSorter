#pragma once
#include "../../BoardDefinitions.hpp"
#include "SensorDefinitions.hpp"

namespace IO {
    
    union Method {
        ServoMotorMethods servoMotorMethod;
        PixelLightMethods pixelLightMethod;
        PhotoResistorMethods photoResistorMethod;
        MotorMethods motorMethods;
        Method();
        Method(ServoMotorMethods methodInvoking);
        Method(PixelLightMethods methodInvoking);
        Method(PhotoResistorMethods methodInvoking);
        Method(MotorMethods methodInvoking);
    };

    struct SensorMessage {
        int sensorID;
        Definition::SensorType type;
        Method method;
        SensorDataTypes data;
        SensorMessage();
        SensorMessage(int id, Definition::SensorType destType, Method methodInvoking, SensorDataTypes dataRequired);
    };

    struct SensorMessageResponse {
        bool wasSuccessful;
        SensorDataTypes data;
        SensorMessageResponse();
        SensorMessageResponse(bool successful, SensorDataTypes dataIn);
    };
}