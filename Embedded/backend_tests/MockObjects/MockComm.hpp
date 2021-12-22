#pragma once

#include "../../backend/src/IO/Comms/IComm.hpp"

namespace MockObject {
    class MockComm : public Comm::IComm {
    public:
        MockComm();
        bool Connect() override;
        void Disconnect() override;
        bool Send(const int address, const MessageProtocol::MessageByteStream& payload) override;
        MessageProtocol::MessageByteStream Recieve() override;
        MessageProtocol::MessageByteStream m_sentPayload;
        MessageProtocol::MessageByteStream m_recvPayload;
    };
}