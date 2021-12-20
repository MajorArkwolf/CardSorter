#pragma once
#include "../../Message.hpp"

namespace Comm { 
    class IComm {
    public:
        virtual ~IComm() {};
        virtual bool Connect() = 0;
        virtual void Disconnect() = 0;
        virtual bool Send(const int address, const MessageProtocol::MessageByteStream& payload) = 0;
        virtual MessageProtocol::MessageByteStream Recieve() = 0;
    };
}