#include "Message.hpp"

#ifdef _ARDUINO_
 #include <Arduino.h>
#else
 #include <string.h>
#endif

namespace MessageProtocol {
    MessageByteStream::MessageByteStream() {
        m_numberOfBytes = 0;
        m_byteStream = nullptr;
    }

    MessageByteStream::~MessageByteStream() {
        free(m_byteStream);
        m_numberOfBytes = 0;
    }

    MessageByteStream::MessageByteStream(size_t numberOfBytes, byte* byteStream) {
        m_numberOfBytes = numberOfBytes;
        m_byteStream = byteStream;
    }

    MessageByteStream::MessageByteStream(const MessageByteStream& messageIn) {
        this->m_numberOfBytes = 0;
        this->m_byteStream = nullptr;
        if (messageIn.GetByteStream() != nullptr && messageIn.GetNumberOfBytes() > 0) {
            this->m_byteStream = (byte*)malloc(messageIn.GetNumberOfBytes());
            if (this->m_byteStream != nullptr) {
                memcpy(&this->m_byteStream, messageIn.GetByteStream(), messageIn.GetNumberOfBytes());
                this->m_numberOfBytes = messageIn.GetNumberOfBytes();
            }
        }
    }

    MessageByteStream MessageByteStream::operator=(const MessageByteStream& messageIn) {
        if (messageIn.GetByteStream() != nullptr && messageIn.GetNumberOfBytes() > 0) {
            byte* byteStream = (byte*)malloc(messageIn.GetNumberOfBytes());
            if (byteStream != nullptr) {
                memcpy(byteStream, messageIn.GetByteStream(), messageIn.GetNumberOfBytes());
                auto numberOfBytes = messageIn.GetNumberOfBytes();
                return MessageByteStream(numberOfBytes, byteStream);
            }
        }
        return MessageByteStream();
    }

    size_t MessageByteStream::GetNumberOfBytes() const {
        return m_numberOfBytes;
    }

    const byte* MessageByteStream::GetByteStream() const {
        return m_byteStream;
    }

    Message::Message() {
        Type = MessageType::Failure;
        NumberOfBytes = 0;
        Data = nullptr;
    }

    Message::~Message() {
        Type = MessageType::Failure;
        NumberOfBytes = 0;
        free(Data);
    }

    Message::Message(MessageType typeOfMessage, const MessageByteStream& payload) {

    }

    Message BytesToMessage(const byte* byteMessage) {
        size_t offset = 0;
        auto output = Message();
        memcpy(&output.Type, &byteMessage[offset], sizeof(MessageType));
        offset += sizeof(MessageType);
        memcpy(&output.NumberOfBytes, &byteMessage[offset], sizeof(uint32_t));
        offset += sizeof(uint32_t);
        if (output.NumberOfBytes > 0) {
            output.Data = (byte*)malloc(output.NumberOfBytes);
            memcpy(output.Data, &byteMessage[offset], sizeof(byte) * output.NumberOfBytes);
        } else {
            output.Data = nullptr;
        }
        return output;
    }

    MessageByteStream MessageToBytes(const Message& message) {
        auto byteStream = MessageByteStream();
        byteStream.numberOfBytes = (sizeof(Message) - sizeof(byte*));
        if (message.Data != nullptr && message.NumberOfBytes > 0) {
            byteStream.numberOfBytes += message.NumberOfBytes;
        }
        byteStream.byteStream = (byte*)malloc(byteStream.numberOfBytes);
        size_t offset = 0;
        memcpy(&byteStream.byteStream[offset], &message.Type, sizeof(MessageType));
        offset += sizeof(MessageType);
        if (message.Data != nullptr && message.NumberOfBytes > 0) {
            memcpy(&byteStream.byteStream[offset], &message.NumberOfBytes, sizeof(uint32_t));
            offset += sizeof(uint32_t);
        } else {
            auto zeroSize = message.NumberOfBytes;
            zeroSize = 0;
            memcpy(&byteStream.byteStream[offset], &zeroSize, sizeof(uint32_t));
            offset += sizeof(uint32_t);
        }
        return byteStream;
    }
}