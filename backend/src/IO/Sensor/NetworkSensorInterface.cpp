#include "NetworkSensorInterface.hpp"
#include "../../Message.hpp"

namespace IO { 
    NetworkSensorInterface::NetworkSensorInterface(int address, Comm::IComm* comm) : m_address(address) {
        m_comm = comm;
    }

    SensorMessageResponse NetworkSensorInterface::SendAndRecieveMessage(const SensorMessage& message) {
        auto messageOut = MessageProtocol::Message(MessageProtocol::MessageType::SensorInstruction, MessageProtocol::GenericMessageToBytes(message)).MessageToBytes();
        m_comm->Send(m_address, messageOut);
        auto messageIn = MessageProtocol::BytesToGenericMessage<SensorMessageResponse>(m_comm->Recieve().GetByteStream());
    }
}