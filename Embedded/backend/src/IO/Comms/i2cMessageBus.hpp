#pragma once
#include "../../Message.hpp"
namespace IO {
    class IOManager;
}

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

        void SetManager(IO::IOManager* messageHandler) { m_messageHandler = messageHandler; }
        IO::IOManager* GetManager() {return m_messageHandler;}
    private:
        MessageProtocol::MessageByteStream m_recv;
        MessageProtocol::MessageByteStream m_send;
        bool m_holdingRecieveMessage;
        bool m_holdingSendMessage;

        IO::IOManager* m_messageHandler;
    };
}