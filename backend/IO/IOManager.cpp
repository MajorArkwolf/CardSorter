#include "IOManager.hpp"
#include "IOFactory.hpp"

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
        auto messageOut = MessageProtocol::Message();
        switch (messageIn.Type)
        {
        case MessageProtocol::MessageType::CreateSensor:
            auto factoryMessage = MessageProtocol::BytesToGenericMessage<IO::Factory::FactoryMessage>(messageIn.Data);
            if (factoryMessage.Type != Definition::SensorType::None) {
                auto sensor = Factory::CreateSensor(factoryMessage);
                if (sensor != nullptr) {
                    m_sensorArray.Append(sensor);
                    break;
                }
            }
            break;
        case MessageProtocol::MessageType::SensorInstruction:
            for (unsigned i = 0; i < m_sensorArray.GetSize(); ++i) {
                auto sensor = m_sensorArray.Get(i);
                if (sensor != nullptr) {
                    if (sensor->GetID() == 0 && sensor->GetSensorType() == ) {

                    }
                }
            }
            break;
        case MessageProtocol::MessageType::Reset:
            Reset();
            messageOut.Type = MessageProtocol::MessageType::Acknowledge;
            break;
        case MessageProtocol::MessageType::Emergency:
        case MessageProtocol::MessageType::Failure:
            /* code */
            break;
        default:
            break;
        }
        return messageOut;
    }

    void IOManager::Update() {

    }
}