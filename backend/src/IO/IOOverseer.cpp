#include "IOOverseer.hpp"
#include "IOFactory.hpp"

#include "Sensor/DeattachedServoMotor.hpp"


namespace IO {
        IOOverseer::IOOverseer() : m_sensorIDDistributor(),
        m_sensorMapping(),
        i2c() {}
        
        IOOverseer::~IOOverseer() {}

        void IOOverseer::Setup() {
            i2c.Connect();
            Serial.println("Step 1");
            RegisterSensor(1, Definition::SensorType::DeattachedServoMotor, SensorInitData(ServoMotorData(3)));
        }

        void IOOverseer::Update() {

        }

        void IOOverseer::RegisterSensor(int boardAddress, Definition::SensorType type, SensorInitData data) {
            if(boardAddress != 0) {
                Serial.println("Step 2");
                SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
                auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, type, newId, data);
                i2c.Send(boardAddress, MessageProtocol::Message(MessageProtocol::MessageType::CreateSensor, MessageProtocol::GenericMessageToBytes(sensorInfo)).MessageToBytes());
                auto response = MessageProtocol::Message::BytesToMessage(i2c.Recieve());
                Serial.println("Step 3");
                if (response.GetMessageType() == MessageProtocol::MessageType::Acknowledge) {
                    auto factory = Factory::IOFactory(&i2c);
                    Serial.println("Step 4");
                    auto networkData = SensorInitData(NSensor(boardAddress));
                    sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Network, type, newId, networkData);
                    m_sensorMapping.Append(factory.CreateSensor(sensorInfo));
                    Serial.println("Big success");
                }
            } else {
                SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
                auto factory = Factory::IOFactory(&i2c);
                auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, type, newId, data);
                auto sensor = factory.CreateSensor(sensorInfo);
                sensor->Setup();
                m_sensorMapping.Append(sensor);
            }
            Serial.println("Step 5");
        }
}