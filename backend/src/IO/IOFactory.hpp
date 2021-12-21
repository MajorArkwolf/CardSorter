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

        struct FactoryMessage {
            SensorLocation Location;
            Definition::SensorType Type;
            SensorID ID;
            SensorInitData Data;
            FactoryMessage();
            FactoryMessage(SensorLocation location, Definition::SensorType type, SensorID id, SensorInitData data);
        };

        Sensor* CreateSensor(const FactoryMessage& message);
    }
}