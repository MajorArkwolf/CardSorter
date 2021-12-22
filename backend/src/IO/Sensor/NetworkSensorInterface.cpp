#include "NetworkSensorInterface.hpp"
#include "../../Message.hpp"

namespace IO { 
    NetworkSensorInterface::NetworkSensorInterface(int address, Comm::IComm* comm) : m_address(address) {
        m_comm = comm;
    }

    SensorMessageResponse NetworkSensorInterface::SendAndRecieveMessage(const SensorMessage& message) {
        auto messageOut = MessageProtocol::Message(MessageProtocol::MessageType::SensorInstruction, MessageProtocol::GenericMessageToBytes(message)).MessageToBytes();
        auto output = SensorMessageResponse();
        m_comm->Send(m_address, messageOut);
        auto payloadRecv = MessageProtocol::Message::BytesToMessage(m_comm->Recieve());
        if (payloadRecv.GetMessageType() == MessageProtocol::MessageType::Acknowledge) {
            auto output = MessageProtocol::BytesToGenericMessage<SensorMessageResponse>(payloadRecv.GetData());
        }
        return output;
    }
}