#include "IOFactory.hpp"

#ifdef _COMPUTER_
  #include <string.h>
#else
  #include <Arduino.h>
#endif


#include "Sensor/NPhotoResistor.hpp"
#include "Sensor/NServoMotor.hpp"
#include "Sensor/NPixelLight.hpp"

#ifndef _COMPUTER_
#include "Sensor/PixelLight.hpp"
#include "Sensor/ServoMotor.hpp"
#include "Sensor/PhotoResistor.hpp"
#include "Sensor/DeattachedServoMotor.hpp"
#endif

namespace {
    IO::Sensor* CreateLocalSensor(const IO::Factory::FactoryMessage& message) {
        IO::Sensor* sensor = nullptr;
        switch (message.Type)
        {
        #ifndef _COMPUTER_
        case IO::Definition::SensorType::PixelLight:
            sensor = new IO::PixelLight(message.ID, message.Data.pixelLightData);
            sensor->Setup();
            break;
        case IO::Definition::SensorType::PhotoResistor:
            sensor = new IO::PhotoResistor(message.ID, message.Data.photoResitorData);
            sensor->Setup();
            break;
        case IO::Definition::SensorType::ServoMotor:
            sensor = new IO::ServoMotor(message.ID, message.Data.servoMotorData);
            sensor->Setup();
            break;
        case IO::Definition::SensorType::DeattachedServoMotor:
            sensor = new IO::DeattachedServoMotor(message.ID, message.Data.servoMotorData);
            sensor->Setup();
        #endif
        default:
            break;
        }
        return sensor;
    }

    IO::Sensor* CreateNetworkSensor(const IO::Factory::FactoryMessage& message) {
        IO::Sensor* sensor = nullptr;
        auto& nSensor = message.Data.networkSensor;
        switch (message.Type)
        {
        case IO::Definition::SensorType::PixelLight:
            sensor = new IO::NPixelLight(message.ID, nSensor.BoardID);
            break;
        case IO::Definition::SensorType::PhotoResistor:
            sensor = new IO::NPhotoResistor(message.ID, nSensor.BoardID);
            break;
        case IO::Definition::SensorType::DeattachedServoMotor:
        case IO::Definition::SensorType::ServoMotor:
            sensor = new IO::NServoMotor(message.ID, nSensor.BoardID);
            break;
        default:
            break;
        }
        return sensor;
    }
}

namespace IO { 
    namespace Factory {
        Sensor* CreateSensor(const FactoryMessage& message) {
            Sensor* sensor = nullptr;
            switch (message.Location)
            {
            case SensorLocation::Local:
                sensor = CreateLocalSensor(message);
                break;
            case SensorLocation::Network:
                sensor = CreateNetworkSensor(message);
                break;
            default:
                break;
            }
            return sensor;
        }
        FactoryMessage::FactoryMessage() {
            Location = SensorLocation::Unknown;
            Type = Definition::SensorType::None;
            ID = 0;
            Data = SensorInitData();
        }
        FactoryMessage::FactoryMessage(SensorLocation location, Definition::SensorType type, SensorID id, SensorInitData data) {
            Location = location;
            Type = type;
            ID = id;
            Data = data;
        }
    }
}