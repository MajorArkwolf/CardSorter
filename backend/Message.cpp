#include "Message.hpp"

#ifdef _ARDUINO_
 #include <Arduino.h>
#else
 #include <string.h>
#endif

namespace MessageProtocol {
    MessageByteStream::MessageByteStream() {
        numberOfBytes = 0;
        byteStream = nullptr;
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