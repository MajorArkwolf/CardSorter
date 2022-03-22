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
        Header header;
        int numOfBytes;
        byte *content;
    };
}