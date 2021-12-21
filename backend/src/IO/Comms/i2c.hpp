#pragma once
#include "IComm.hpp"

namespace IO {
    class IOManager;
}

namespace Comm {
    class i2cDriver : public IComm {
    public:
        i2cDriver();
        ~i2cDriver() override;
        bool Connect() override;
        void Disconnect() override;
        bool Send(const int address, const MessageProtocol::MessageByteStream& payload) override;
        MessageProtocol::MessageByteStream Recieve() override;
    private:
        int m_lastSendAddress;
        bool m_isConnected;
    };

    class i2cPassenger : public IComm {
    public:
        i2cPassenger(int address);
        ~i2cPassenger() override;
        bool Connect() override;
        void Disconnect() override;
        bool Send(const int address, const MessageProtocol::MessageByteStream& payload) override;
        MessageProtocol::MessageByteStream Recieve() override;
        bool MessagePending();
        static void OnRequestHandle();
        static void OnRecieveHandle(int numBytes);
        static void SetMessanger(IO::IOManager* manager);
    private:
        int m_address;
        bool m_isConnected;
    };
}