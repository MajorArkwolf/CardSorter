#pragma once
#include "../../Message.hpp"

namespace Comm {
    class i2cMessageBus {
    public:
        i2cMessageBus();
        bool HoldingRecieveMessage();
        bool HoldingSendMessage();

        bool GetRecvMessage(MessageProtocol::MessageByteStream& out);
        bool GetSendMessage(MessageProtocol::MessageByteStream& out);

        bool SetRecvMessage(MessageProtocol::MessageByteStream& in);
        bool SetSendMessage(MessageProtocol::MessageByteStream& in);

    private:
        MessageProtocol::MessageByteStream m_recv;
        MessageProtocol::MessageByteStream m_send;
        bool m_holdingRecieveMessage;
        bool m_holdingSendMessage;
    };
}