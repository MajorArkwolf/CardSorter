#include "IOOverseer.hpp"
#include "IOFactory.hpp"

#include "Sensor/DeattachedServoMotor.hpp"


namespace IO {
        IOOverseer::IOOverseer() : m_sensorIDDistributor() {
            m_sensorMapping = Container::Array<Sensor*>();
        }
        
        IOOverseer::~IOOverseer() {}

        void IOOverseer::Setup() {
            auto data = SensorInitData(ServoMotorData(3));
            RegisterSensor(0, Definition::SensorType::DeattachedServoMotor, data);
        }

        void IOOverseer::Update() {
            auto motor = (IServoMotor*)m_sensorMapping.Get(0);
            auto lastValue = motor->Get();
            auto newValue = lastValue + 45;
            if (newValue > 180) {
                newValue = 0;
            }
            Serial.print(lastValue);
            Serial.print(" ");
            Serial.print(newValue);
            Serial.println();
            motor->Set(newValue);
        }

        void IOOverseer::RegisterSensor(int boardAddress, Definition::SensorType type, SensorInitData data) {
            if(boardAddress != 0) {
                SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
                auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, type, newId, data);
                // Request creation over network
                auto networkData = SensorInitData(NSensor(boardAddress));
                sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Network, type, newId, networkData);
                m_sensorMapping.Append(Factory::CreateSensor(sensorInfo));

            } else {
                SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
                auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, type, newId, data);
                auto sensor = Factory::CreateSensor(sensorInfo);
                sensor->Setup();
                m_sensorMapping.Append(sensor);
            }
        }
}