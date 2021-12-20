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
        if (messageIn.GetByteStream() != nullptr) {
            this->m_byteStream = (byte*)malloc(messageIn.GetNumberOfBytes());
            if (this->m_byteStream != nullptr) {
                memcpy(&this->m_byteStream, messageIn.GetByteStream(), messageIn.GetNumberOfBytes());
                this->m_numberOfBytes = messageIn.GetNumberOfBytes();
            }
        }
    }

    MessageByteStream MessageByteStream::operator=(const MessageByteStream& messageIn) {
        if (messageIn.GetByteStream() != nullptr) {
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
        m_type = MessageType::Failure;
        m_numberOfBytes = 0;
        m_data = nullptr;
    }

    Message::~Message() {
        free(m_data);
    }

    Message::Message(MessageType typeOfMessage, const MessageByteStream& payload) {
        m_type = typeOfMessage;
        if (payload.GetNumberOfBytes() <= 0 || payload.GetByteStream() == nullptr) {
            m_numberOfBytes = 0;
            m_data = nullptr;
        } else {
            m_data = (byte*)malloc(payload.GetNumberOfBytes());
            if (m_data != nullptr) {
                memcpy(m_data, payload.GetByteStream(), payload.GetNumberOfBytes());
                m_numberOfBytes = payload.GetNumberOfBytes();
            } else {
                m_numberOfBytes = 0;
            }
        }
        
    }

    Message::Message(MessageType typeOfMessage, size_t numberOfBytes, byte* byteStream) {
        m_type = typeOfMessage;
        m_numberOfBytes = numberOfBytes;
        m_data = byteStream;
    }

    MessageType Message::GetMessageType() const {
        return m_type;
    }

    size_t Message::GetNumberOfBytes() const {
        return m_numberOfBytes;
    }

    const byte* Message::GetData() const {
        return m_data;
    }

    MessageByteStream Message::MessageToBytes() {
        size_t payloadByteSize = this->m_numberOfBytes;
        if (this->m_data == nullptr) {
            payloadByteSize = 0;
        }
        // We dont want our pointer for our payload to be part of the bytestream and only its contents.
        size_t numberOfBytes = (sizeof(Message) - sizeof(byte*)) + payloadByteSize;
        byte* outputByteStream = (byte*)malloc(numberOfBytes);

        if (outputByteStream != nullptr) {
            size_t offset = 0;
            memcpy(&outputByteStream[offset], &this->m_type, sizeof(MessageType));
            offset += sizeof(MessageType);
            memcpy(&outputByteStream[offset], &payloadByteSize, sizeof(size_t));
            offset += sizeof(size_t);
            if (payloadByteSize > 0) {
                memcpy(&outputByteStream[offset], this->m_data, payloadByteSize);
            }
        }
        return MessageByteStream(numberOfBytes, outputByteStream);
    }

    Message Message::BytesToMessage(const MessageByteStream& byteMessage) {
        size_t offset = 0;
        MessageType type = MessageType::Failure;
        size_t numberOfBytes = 0;
        byte* data = nullptr;

        auto dataStreamIn = byteMessage.GetByteStream();

        memcpy(&type, dataStreamIn, sizeof(MessageType));
        offset += sizeof(MessageType);
        memcpy(&numberOfBytes, &byteMessage.GetByteStream()[offset], sizeof(size_t));
        offset += sizeof(size_t);
        if (numberOfBytes > 0) {
            data = (byte*)malloc(numberOfBytes);
            if (data != nullptr) {
                memcpy(data, &byteMessage.GetByteStream()[offset], numberOfBytes);
            } else {
                numberOfBytes = 0;
            }
        }
        return Message(type, numberOfBytes, data);
    }
}