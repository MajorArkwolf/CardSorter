#include "IOManager.hpp"
#include "IOFactory.hpp"
#include "Sensor/SensorMessage.hpp"

namespace IO {
    IOManager::IOManager(int thisBoardID) :
    i2c(thisBoardID)
    {
        m_sensorArray = Container::Array<Sensor*>();
        Comm::i2cPassenger::SetMessanger(this);
    }

    IOManager::~IOManager() {
        Reset();
    }

    void IOManager::Setup() {
        i2c.Connect();
    }

    void IOManager::Update() {}

    void IOManager::Reset() {
        for (unsigned i = 0; i < m_sensorArray.GetSize(); ++i) {
            auto sensor = m_sensorArray.Get(i);
            delete(sensor);
        }
        m_sensorArray = Container::Array<Sensor*>();
    }

    MessageProtocol::Message IOManager::HandleMessage(const MessageProtocol::Message& messageIn) {
        Serial.println("Message in");
        switch (messageIn.GetMessageType())
        {
        case MessageProtocol::MessageType::CreateSensor:
            Serial.println("creating sensor");
            return CreateSensorRequest(messageIn);
        case MessageProtocol::MessageType::SensorInstruction:
            return SensorInstructionRequest(messageIn);
        case MessageProtocol::MessageType::Reset:
            Reset();
            return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
        case MessageProtocol::MessageType::Emergency:
        case MessageProtocol::MessageType::Failure:
            /* code */
            break;
        default:
            break;
        }
        return MessageProtocol::Message();
    }

    MessageProtocol::Message IOManager::CreateSensorRequest(const MessageProtocol::Message& messageIn) {
        auto factoryMessage = MessageProtocol::BytesToGenericMessage<IO::Factory::FactoryMessage>(messageIn.GetData());
        if (factoryMessage.Type != Definition::SensorType::None) {
            auto factory = Factory::IOFactory();
            auto sensor = factory.CreateSensor(factoryMessage);
            if (sensor != nullptr) {
                m_sensorArray.Append(sensor);
                return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
            }
        }
        return MessageProtocol::Message();
    }

    MessageProtocol::Message IOManager::SensorInstructionRequest(const MessageProtocol::Message& messageIn) {
        auto sensorMessage = MessageProtocol::BytesToGenericMessage<SensorMessage>(messageIn.GetData());
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
        return MessageProtocol::Message();
    }
}