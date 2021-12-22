#pragma once
#include "Comms/IComm.hpp"
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

        class IOFactory {
        public:
            IOFactory();
            IOFactory(Comm::IComm* networkModule);
            Sensor* CreateSensor(const FactoryMessage& message);
        private:
            IO::Sensor* CreateLocalSensor(const FactoryMessage& message);
            IO::Sensor* CreateNetworkSensor(const FactoryMessage& message);
            Comm::IComm* m_networkModule;
        };
    }
}