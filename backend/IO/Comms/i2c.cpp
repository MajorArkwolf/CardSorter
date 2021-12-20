#include "i2c.hpp"
#include <Wire.h>

namespace Comm {
    static i2cMessageBus messageContainer = i2cMessageBus();

    i2cMessageBus::i2cMessageBus() :
        m_recv(),
        m_send(),
        m_holdingSendMessage(false),
        m_holdingRecieveMessage(false) {}

    bool i2cMessageBus::HoldingRecieveMessage() {
        return m_holdingRecieveMessage;
    }

    bool i2cMessageBus::HoldingSendMessage() {
        return m_holdingSendMessage;
    }

    bool i2cMessageBus::GetRecvMessage(MessageProtocol::MessageByteStream& out) {
        if (m_holdingRecieveMessage) {
            MessageProtocol::MessageByteStream::Swap(m_recv, out);
            m_holdingRecieveMessage = false;
            return true;
        }
        return false;
    }

    bool i2cMessageBus::GetSendMessage(MessageProtocol::MessageByteStream& out) {
        if (m_holdingSendMessage) {
            MessageProtocol::MessageByteStream::Swap(m_send, out);
            m_holdingSendMessage = false;
            return true;
        }
        return false;
    }

    bool i2cMessageBus::SetRecvMessage(MessageProtocol::MessageByteStream& in) {
        if (!m_holdingRecieveMessage) {
            MessageProtocol::MessageByteStream::Swap(m_recv, in);
            m_holdingRecieveMessage = true;
            return true;
        }
        return false;
    }

    bool i2cMessageBus::SetSendMessage(MessageProtocol::MessageByteStream& in) {
        if (!m_holdingSendMessage) {
            MessageProtocol::MessageByteStream::Swap(m_send, in);
            m_holdingSendMessage = true;
            return true;
        }
        return false;
    }

    i2c::i2c(int address) :
        m_isConnected(false),
        m_address(address) 
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
    }

    MessageProtocol::MessageByteStream i2c::Recieve() {
        
    }

    static void i2c::OnRequestHandle() {
        //Insert message handler here
    }

    static void i2c::OnRecieveHandle(int howMany) {
        size_t bytesIn = howMany;
        size_t offset = 0;
        byte* data = (byte*)malloc(bytesIn);
        while (Wire.avaliable()) {
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