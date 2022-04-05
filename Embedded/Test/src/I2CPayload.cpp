#include "I2CPayload.h"

namespace Ark::Network {
    Payload::~Payload() {
        delete(header);
    }

    static Payload GeneratePayload(int numberOfBytes, byte* byteArray) {
        
    }
}