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
        MessageByteStream operator=(const MessageByteStream& messageIn);
        size_t GetNumberOfBytes() const;
        const byte* GetByteStream() const;
    private:
        size_t m_numberOfBytes;
        byte* m_byteStream;
    };

    class Message {
        MessageType Type;
        uint32_t NumberOfBytes;
        byte* Data;

        Message();
        ~Message();
        Message(MessageType typeOfMessage, const MessageByteStream& payload);
    };

    Message BytesToMessage(const byte* byteMessage);

    MessageByteStream MessageToBytes(const Message& message);

    template<typename T>
    MessageByteStream GenericMessageToBytes(T message) {
        auto mesageBytestream = MessageProtocol::MessageByteStream();
        mesageBytestream.numberOfBytes = sizeof(T);
        mesageBytestream.byteStream = (byte*)malloc(mesageBytestream.numberOfBytes);
        memcpy(mesageBytestream.byteStream, &message, mesageBytestream.numberOfBytes);
        return mesageBytestream;
    }

    template<class T>
    T BytesToGenericMessage(const byte* byteMessage) {
        T message;
        memcpy(&message,  byteMessage, sizeof(T));
        return message;
    }
}