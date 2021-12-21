#include "i2c.hpp"
#include <Wire.h>
#include "i2cMessageBus.hpp"
#include "../IOManager.hpp"

namespace Comm {
    static auto messageContainer = Comm::i2cMessageBus();
    static auto g_requestToken = false;

    void i2cPassenger::SetMessanger(IO::IOManager* manager) {
        messageContainer.SetManager(manager);
    }

    // i2cPassenger Implementation

    i2cPassenger::i2cPassenger(int address) : 
    m_address(address) {}

    i2cPassenger::~i2cPassenger() {
        Disconnect();
    }

    bool i2cPassenger::Connect() {
        Wire.begin(m_address);
        Wire.onReceive(i2cPassenger::OnRecieveHandle);
        Wire.onRequest(i2cPassenger::OnRequestHandle);
        m_isConnected = true;
        return m_isConnected;
    }

    void i2cPassenger::Disconnect() {
        m_isConnected= false;
        Wire.onReceive(nullptr);
        Wire.onRequest(nullptr);
    }

    bool i2cPassenger::Send(const int address, const MessageProtocol::MessageByteStream& payload) {

    }

    MessageProtocol::MessageByteStream i2cPassenger::Recieve() {
        auto messageOut = MessageProtocol::MessageByteStream();
        if (messageContainer.HoldingRecieveMessage()) {
            messageContainer.GetRecvMessage(messageOut);
        }
        return messageOut;
    }

    bool i2cPassenger::MessagePending() {
        return messageContainer.HoldingRecieveMessage();
    }

    void i2cPassenger::OnRequestHandle() {
        auto messageIn = MessageProtocol::MessageByteStream();
        if (messageContainer.HoldingRecieveMessage()) {
            messageContainer.GetRecvMessage(messageIn);
            if (messageIn.GetByteStream() != nullptr && messageContainer.GetManager() != nullptr) {
                auto messageOut = messageContainer.GetManager()->HandleMessage(MessageProtocol::Message::BytesToMessage(messageIn)).MessageToBytes();
                size_t messageSize = messageOut.GetNumberOfBytes();
                messageContainer.SetSendMessage(messageOut);
                byte* mes = (byte*)(&messageSize);
                Wire.write(mes, sizeof(messageSize));
                return;
            }
        } else if (messageContainer.HoldingSendMessage()) {
            messageContainer.GetSendMessage(messageIn);
            Wire.write(messageIn.GetByteStream(), messageIn.GetNumberOfBytes());
        }
    }

    void i2cPassenger::OnRecieveHandle(int howMany) {
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

    // i2cDriver Implementation

    i2cDriver::i2cDriver() : 
    m_lastSendAddress(0),
    m_isConnected(false) {}

    i2cDriver::~i2cDriver() {
        Disconnect();
    }

    bool i2cDriver::Connect() {
        Wire.begin();
        m_isConnected = true;
        return m_isConnected;
    }

    void i2cDriver::Disconnect() {
        m_isConnected= false;
    }

    bool i2cDriver::Send(const int address, const MessageProtocol::MessageByteStream& payload) {
        Wire.beginTransmission(address);
        Wire.write(payload.GetByteStream(), payload.GetNumberOfBytes());
        Wire.endTransmission();
        m_lastSendAddress = address;
    }

    MessageProtocol::MessageByteStream i2cDriver::Recieve() {
        byte* payLoad = nullptr;
        size_t numberOfBytes = 0;
        Serial.println("start recv");
        if (m_lastSendAddress != 0) {
            Serial.println("requesting payload size");
            size_t offset = 0;
            byte* mes = (byte*)(&numberOfBytes);
            Wire.requestFrom(m_lastSendAddress, sizeof(numberOfBytes));
            while(Wire.available())    // slave may send less than requested
            {
                Serial.println("Reading...");
                if (offset >= (size_t)(sizeof(numberOfBytes))) {
                    Serial.println("to many bites, ejecting");
                    Serial.println(offset);
                    Serial.println(sizeof(numberOfBytes) / sizeof(byte));
                    break;
                }
                mes[offset] = Wire.read();
                Serial.println(numberOfBytes);
                Serial.println("Read.");
                ++offset;
            }
            payLoad = (byte*)malloc(numberOfBytes);
            Serial.println(numberOfBytes);
            if (payLoad != nullptr) {
                offset = 0;
                Serial.println("second request");
                Wire.requestFrom(m_lastSendAddress, numberOfBytes);
                while(Wire.available())    // slave may send less than requested
                {
                    Serial.println("Go");
                    payLoad[offset] = Wire.read();
                    ++offset;
                }
            } else {
                numberOfBytes = 0;
            }
        }
        auto messageOut = MessageProtocol::MessageByteStream(numberOfBytes, payLoad);
        m_lastSendAddress = 0;
        Serial.println("recv finsihed");
        return messageOut;
    }
}