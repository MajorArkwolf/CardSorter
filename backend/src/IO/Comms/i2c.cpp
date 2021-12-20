#include "i2c.hpp"
#include <Wire.h>
#include "i2cMessageBus.hpp"

namespace Comm {
    static auto messageContainer = Comm::i2cMessageBus();

    i2c::i2c(int address) :
        m_address(address),
        m_isConnected(false)
        {
        if ( address != 0 ) {
            m_commType = CommType::Passenger;
        } else {
            m_commType = CommType::Driver;
        }
    }

    i2c::~i2c() {

    }

    bool i2c::Connect() {
        if (m_commType == CommType::Driver) {
            Wire.begin();
        } else {
            Wire.begin(m_address);
            Wire.onReceive(i2c::OnRecieveHandle);
            Wire.onRequest(i2c::OnRequestHandle);
        }
        m_isConnected = true;
        return m_isConnected;
    }

    void i2c::Disconnect() {
        m_isConnected= false;
    }

    bool i2c::Send(const int address, const MessageProtocol::MessageByteStream& payload) {
        Wire.beginTransmission(address);
        Wire.write(payload.GetByteStream(), payload.GetNumberOfBytes());
        Wire.endTransmission();
    }

    MessageProtocol::MessageByteStream i2c::Recieve() {
        auto messageOut = MessageProtocol::MessageByteStream();
        if (messageContainer.HoldingRecieveMessage()) {
            messageContainer.GetRecvMessage(messageOut);
        }
        return messageOut;
    }

    void i2c::OnRequestHandle() {
        //Insert message handler here
    }

    void i2c::OnRecieveHandle(int howMany) {
        size_t bytesIn = howMany;
        size_t offset = 0;
        byte* data = (byte*)malloc(bytesIn);
        while (Wire.available()) {
            if (offset >= howMany) {
                break;
            }
            data[offset] = Wire.read();
            ++offset;
        }

        auto messageIn = MessageProtocol::MessageByteStream(bytesIn, data);
        if (!messageContainer.HoldingRecieveMessage()) {
            messageContainer.SetRecvMessage(messageIn);
        } else {
            free(data);
        }
    }
}