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
            RegisterSensor(1, Definition::SensorType::DeattachedServoMotor, SensorInitData(ServoMotorData(3)));
        }

        void IOOverseer::Update() {
            if (m_sensorMapping.GetSize() > 0) {
                auto sensor = m_sensorMapping.Get(0);
                if(sensor != nullptr) {
                    auto type = sensor->GetSensorType();
                    if (type == Definition::SensorType::ServoMotor) {
                        IServoMotor *servo = (IServoMotor*)sensor;
                        int lastPos = servo->Get();
                        auto newPos = lastPos + 20;
                        if (newPos > 180) {
                            newPos = 0;
                        }
                    }
                }
            }
        }

        void IOOverseer::RegisterSensor(int boardAddress, Definition::SensorType type, SensorInitData data) {
            if(boardAddress != 0) {
                SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
                auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, type, newId, data);
                i2c.Send(boardAddress, MessageProtocol::Message(MessageProtocol::MessageType::CreateSensor, MessageProtocol::GenericMessageToBytes(sensorInfo)).MessageToBytes());
                auto response = MessageProtocol::Message::BytesToMessage(i2c.Recieve());
                if (response.GetMessageType() == MessageProtocol::MessageType::Acknowledge) {
                    auto factory = Factory::IOFactory(&i2c);
                    auto networkData = SensorInitData(NSensor(boardAddress));
                    sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Network, type, newId, networkData);
                    m_sensorMapping.Append(factory.CreateSensor(sensorInfo));
                }
            } else {
                SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
                auto factory = Factory::IOFactory(&i2c);
                auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, type, newId, data);
                auto sensor = factory.CreateSensor(sensorInfo);
                sensor->Setup();
                m_sensorMapping.Append(sensor);
            }
        }
}