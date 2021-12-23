#include "IOManager.hpp"
#include "IOFactory.hpp"
#include "Sensor/SensorMessage.hpp"
#include "Sensor/Sensor.hpp"
#include "../Error.hpp"

namespace {
    bool IDAlreadyInUse(const Container::Array<IO::Sensor*>& container, IO::SensorID id) {
        for (auto i = 0; i < container.GetSize(); ++i) {
            if (container.Get(i)->GetID() == id) {
                return true;
            }
        }
        return false;
    }
}

namespace IO {
    IOManager::IOManager(int thisBoardID) :
    i2c(thisBoardID),
    m_systemFailure(false)
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
        switch (messageIn.GetMessageType())
        {
        case MessageProtocol::MessageType::CreateSensor:
            return CreateSensorRequest(messageIn);
        case MessageProtocol::MessageType::SensorInstruction:
            return SensorInstructionRequest(messageIn);
        case MessageProtocol::MessageType::Reset:
            Reset();
            m_systemFailure = false;
            return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
        case MessageProtocol::MessageType::Emergency:
        case MessageProtocol::MessageType::Failure:
            m_systemFailure = true;
            return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
        default:
            return MessageProtocol::Message(MessageProtocol::MessageType::Failure, MessageProtocol::GenericMessageToBytes(Comm::Error::Undefined));
            break;
        }
        return MessageProtocol::Message();
    }

    MessageProtocol::Message IOManager::CreateSensorRequest(const MessageProtocol::Message& messageIn) {
        auto factoryMessage = MessageProtocol::BytesToGenericMessage<IO::Factory::FactoryMessage>(messageIn.GetData());
        if (factoryMessage.Type != Definition::SensorType::None) {
            if (!IDAlreadyInUse(m_sensorArray, factoryMessage.ID)) {
                auto factory = Factory::IOFactory();
                auto sensor = factory.CreateSensor(factoryMessage);
                if (sensor != nullptr) {
                    m_sensorArray.Append(sensor);
                    return MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::MessageByteStream());
                }
            } else {
                MessageProtocol::Message(MessageProtocol::MessageType::Failure, MessageProtocol::GenericMessageToBytes(Comm::Error::IDinUse));
            }
        }
        return MessageProtocol::Message(MessageProtocol::MessageType::Failure, MessageProtocol::GenericMessageToBytes(Comm::Error::Undefined));
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