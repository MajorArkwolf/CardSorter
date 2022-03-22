#pragma once

namespace Ark {
    using byte = unsigned char;

    class IPlugin {
        public:
            virtual ~IPlugin() {};
            virtual bool Write(byte argc, byte *argv) = 0;
            virtual byte* Read(byte argc, byte *argv) = 0;
    };
}