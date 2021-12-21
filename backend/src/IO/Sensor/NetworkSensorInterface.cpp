#include "NetworkSensorInterface.hpp"
#include "../../Message.hpp"

namespace IO { 
    NetworkSensorInterface::NetworkSensorInterface(Comm::IComm* comm) {
        m_comm = comm;
    }

    SensorMessageResponse NetworkSensorInterface::SendAndRecieveMessage(int address, const SensorMessage& message) {
        auto messageOut = MessageProtocol::Message(MessageProtocol::MessageType::SensorInstruction, MessageProtocol::GenericMessageToBytes(message)).MessageToBytes();
        m_comm->Send(address, messageOut);
        auto messageIn = MessageProtocol::BytesToGenericMessage<SensorMessageResponse>(m_comm->Recieve().GetByteStream());
    }
}