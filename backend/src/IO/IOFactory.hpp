#pragma once
#include "../Message.hpp"
#include "Sensor/Sensor.hpp"
#include "Sensor/IPixelLight.hpp"
#include "Sensor/IServoMotor.hpp"
#include "Sensor/IPhotoResistor.hpp"

namespace IO { 
    namespace Factory {
        enum class SensorLocation {
            Local,
            Network,
            Unknown
        };

        union SensorData
        {
            NSensor networkSensor;
            PhotoResitorData photoResitorData;
            ServoMotorData servoMotorData;
            PixelLightData pixelLightData;
            SensorData();
        };

        struct FactoryMessage {
            SensorLocation Location;
            Definition::SensorType Type;
            int ID;
            SensorData Data;
        };

        Sensor* CreateSensor(const FactoryMessage& message);
    }
}