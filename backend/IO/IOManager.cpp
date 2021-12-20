#include "IOManager.hpp"
#include "IOFactory.hpp"
#include "Sensor/SensorMessage.hpp"

namespace IO {
    IOManager::IOManager() {
        m_sensorArray = Container::Array<Sensor*>();
    }

    IOManager::~IOManager() {
        Reset();
    }

    void IOManager::Setup() {

    }

    void IOManager::Reset() {
        for (unsigned i = 0; i < m_sensorArray.GetSize(); ++i) {
            auto sensor = m_sensorArray.Get(i);
            delete(sensor);
        }
        m_sensorArray = Container::Array<Sensor*>();
    }

    MessageProtocol::Message IOManager::HandleMessage(const MessageProtocol::Message& messageIn) {
        switch (messageIn.Type)
        {
        case MessageProtocol::MessageType::CreateSensor:
            auto factoryMessage = MessageProtocol::BytesToGenericMessage<IO::Factory::FactoryMessage>(messageIn.Data);
            if (factoryMessage.Type != Definition::SensorType::None) {
                auto sensor = Factory::CreateSensor(factoryMessage);
                if (sensor != nullptr) {
                    m_sensorArray.Append(sensor);
                    return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
                }
            }
            break;
        case MessageProtocol::MessageType::SensorInstruction:
            auto sensorMessage = MessageProtocol::BytesToGenericMessage<SensorMessage>(messageIn.Data);
            for (unsigned i = 0; i < m_sensorArray.GetSize(); ++i) {
                auto sensor = m_sensorArray.Get(i);
                if (sensor != nullptr) {
                    if (sensor->GetID() == sensorMessage.sensorID && sensor->GetSensorType() == sensorMessage.type) {
                        auto response = sensor->HandleMessage(sensorMessage);
                        auto convertedMessage = MessageProtocol::GenericMessageToBytes(response);
                        return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, convertedMessage);
                    }
                }
            }
            break;
        case MessageProtocol::MessageType::Reset:
            Reset();
            return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
            break;
        case MessageProtocol::MessageType::Emergency:
        case MessageProtocol::MessageType::Failure:
            /* code */
            break;
        default:
            break;
        }
        return MessageProtocol::Message();
    }

    void IOManager::Update() {

    }
}