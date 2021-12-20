#pragma once
#include <stdlib.h>
#include <stdint.h>
#include "BoardDefinitions.hpp"


namespace MessageProtocol {
    enum class MessageType {    
        CreateSensor,
        SensorInstruction,
        Acknowledge,
        Reset,
        Emergency,
        Failure
    };

    class MessageByteStream {
    public:
        MessageByteStream();
        ~MessageByteStream();
        MessageByteStream(size_t numberOfBytes, byte* byteStream);
        MessageByteStream(const MessageByteStream& messageIn);
        MessageByteStream& operator=(const MessageByteStream& messageIn);
        size_t GetNumberOfBytes() const;
        const byte* GetByteStream() const;
    private:
        size_t m_numberOfBytes;
        byte* m_byteStream;
    };

    class Message {
    public:
        Message();
        ~Message();
        Message(MessageType typeOfMessage, const MessageByteStream& payload);
        Message(MessageType typeOfMessage, size_t numberOfBytes, byte* byteStream);
        Message(const Message& messageIn);
        Message& operator=(const Message& messageIn);
        MessageType GetMessageType() const;
        size_t GetNumberOfBytes() const;
        const byte* GetData() const;

        static Message BytesToMessage(const MessageByteStream& byteMessage);

        MessageByteStream MessageToBytes();
    private:
        MessageType m_type;
        size_t m_numberOfBytes;
        byte* m_data;
    };

    template<typename T>
    MessageByteStream GenericMessageToBytes(T message) {
        size_t numberOfBytes = sizeof(T);
        byte* byteStream = (byte*)malloc(numberOfBytes);
        if (byteStream != nullptr) {
            memcpy(byteStream, &message, numberOfBytes);
        }
        return MessageProtocol::MessageByteStream(numberOfBytes, byteStream);
    }

    template<class T>
    T BytesToGenericMessage(const byte* byteMessage) {
        T message;
        memcpy(&message, byteMessage, sizeof(T));
        return message;
    }
}