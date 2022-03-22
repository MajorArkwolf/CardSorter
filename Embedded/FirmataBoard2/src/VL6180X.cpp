#include "VL6180X.h"
#include <Adafruit_VL6180X.h>

namespace {
    Ark::VL6180X::StatusCode ConvertErrorCode(uint8_t status) {
        auto output = Ark::VL6180X::StatusCode::Unknown;
        if (status == VL6180X_ERROR_NONE) {
            output = Ark::VL6180X::StatusCode::Ok;
        }
        else if ((status >= VL6180X_ERROR_SYSERR_1) && (status <= VL6180X_ERROR_SYSERR_5)) {
            output = Ark::VL6180X::StatusCode::SystemError;
        }
        else if (status == VL6180X_ERROR_ECEFAIL) {
            output = Ark::VL6180X::StatusCode::EceFail;
        }
        else if (status == VL6180X_ERROR_NOCONVERGE) {
            output = Ark::VL6180X::StatusCode::NoConvergence;
        }
        else if (status == VL6180X_ERROR_RANGEIGNORE) {
            output = Ark::VL6180X::StatusCode::RangeIgnore;
        }
        else if (status == VL6180X_ERROR_SNR) {
            output = Ark::VL6180X::StatusCode::SignalError;
        }
        else if (status == VL6180X_ERROR_RAWUFLOW) {
            output = Ark::VL6180X::StatusCode::RawUnderflow;
        }
        else if (status == VL6180X_ERROR_RAWOFLOW) {
            output = Ark::VL6180X::StatusCode::RawOverflow;
        }
        else if (status == VL6180X_ERROR_RANGEUFLOW) {
            output = Ark::VL6180X::StatusCode::RangeUnderflow;
        }
        else if (status == VL6180X_ERROR_RANGEOFLOW) {
            output = Ark::VL6180X::StatusCode::RangeOverflow;
        }
        return output;
    }

    byte* ConstructPayload(Ark::VL6180X::StatusCode statusCode, uint8_t range) {
        auto output = new byte[2];
        if (output != nullptr) {
            output[0] = static_cast<uint8_t>(statusCode);
            output[1] = range;
        }
        return output;
    }
}


namespace Ark {

    VL6180X::VL6180X(byte argc, byte *argv) {
        m_controller = new Adafruit_VL6180X();
        if (m_controller != nullptr) {
            delay(1);
            if (!m_controller->begin()) {
                delete (m_controller);
            }
        }
    }

    VL6180X::~VL6180X() {
        delete (m_controller);
    }

    bool VL6180X::Write(byte argc, byte *argv) {
        return false;
    }

    byte* VL6180X::Read(byte argc, byte *argv) {
        if (m_controller == nullptr) {
            return nullptr;
        }

        uint8_t range = m_controller->readRange();
        uint8_t status = m_controller->readRangeStatus();

        return ConstructPayload(ConvertErrorCode(status), range);
    }
}