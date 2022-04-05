#pragma once

namespace Ark::Network {

    enum class Header
    {
        Query = 0,
        Get = 1,
        Set = 2,
        Response = 3,
    };

    struct Payload {
        ~Payload();
        Header *header;
        int *numOfBytes;
        byte *content;
    };

    /**
    * Generates a generic Payload object used by the devices to communicate.
    * Payload object will take complete ownership of the heap allocated byte array.
    */
    static Payload GeneratePayload(int numberOfBytes, byte* byteArray);
}