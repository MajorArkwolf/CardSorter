#include "MockComm.hpp"

namespace MockObject {

    MockComm::MockComm() {
        m_sentPayload = MessageProtocol::MessageByteStream();
        m_recvPayload = MessageProtocol::MessageByteStream();
    }

    bool MockComm::Connect() {
        return true;
    }

    void MockComm::Disconnect() {

    }

    bool MockComm::Send(const int address, const MessageProtocol::MessageByteStream& payload) {
        m_sentPayload = payload;
        return true;
    }

    MessageProtocol::MessageByteStream MockComm::Recieve() {
        auto temp = MessageProtocol::MessageByteStream();
        MessageProtocol::MessageByteStream::Swap(temp, m_recvPayload);
        return temp;
    }
}