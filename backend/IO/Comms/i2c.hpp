#pragma once
#include "IComm.hpp"

namespace Comm {
    enum class CommType {
        Driver,
        Passenger
    };

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
        bool m_holdingRecieveMessage;
        bool m_holdingSendMessage;
        MessageProtocol::MessageByteStream m_recv;
        MessageProtocol::MessageByteStream m_send;
    }

    class i2c : public IComm {
    public:
        i2c(int address);
        ~i2c() override;
        bool Connect() override;
        void Disconnect() override;
        bool Send(const int address, const MessageProtocol::MessageByteStream& payload) override;
        MessageProtocol::MessageByteStream Recieve() override;

        static void OnRequestHandle();
        static void OnRecieveHandle(int numBytes);

    private:
        bool m_isConnected;
        int m_address;
        CommType m_commType;
    };
}