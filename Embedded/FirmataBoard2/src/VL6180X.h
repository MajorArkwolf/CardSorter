#pragma once
#include "IPlugin.h"

class Adafruit_VL6180X;

namespace Ark {

    /**
     * @brief Custom plugin to interact with the Adafruit VL6180X.
     * 
     */
    class VL6180X : public IPlugin {
        public:
            // Status codes will be converted to u8s during transmission.
            enum class StatusCode
            {
                Ok = 0,
                Unknown = 1,
                SystemError = 2,
                EceFail = 3,
                NoConvergence = 4,
                RangeIgnore = 5,
                SignalError = 6,
                RawUnderflow = 7,
                RawOverflow = 8,
                RangeUnderflow = 9,
                RangeOverflow = 10,
            };

            VL6180X(byte argc, byte *argv);

            ~VL6180X() override;

            /**
             * @brief Unused for this object.
             * @param bytes unused
             * @return true (never)
             * @return false will always return false as no operation will occur.
             */
            bool Write(byte argc, byte *argv) override;

            /**
             * @brief Read the status code and range from the VL6180X board.
             * @param bytes parameter is unused as there is only 1 operation required.
             * @return byte* Returns 2 bytes, first byte will be the status code as a u8 and second
             * byte is the range in mm as a u8.
             */
            byte* Read(byte argc, byte *argv) override;

        private:
            Adafruit_VL6180X* m_controller = nullptr;
    };

}