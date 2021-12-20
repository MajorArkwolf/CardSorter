#pragma once
#include "IComm.hpp"

namespace Comm {
    enum class CommType {
        Driver,
        Passenger
    };

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
        int m_address;
        CommType m_commType;
        bool m_isConnected;
    };
}