#pragma once
#include "IComm.hpp"

namespace Comm {
    class SerialComm : public IComm {
    public:
        SerialComm();
        bool Connect() override;
        void Disconnect() override;
        bool Send(const int address, const MessageProtocol::MessageByteStream& payload) override;
        bool Send(const MessageProtocol::MessageByteStream& payload);
        MessageProtocol::MessageByteStream Recieve() override;
    };
}