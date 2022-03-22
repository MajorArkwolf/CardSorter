#pragma once
#include "IPlugin.h"
namespace Ark {
    /**
     * @brief Plugin protocol works on the follow permise
     * The first byte is the header and defines the operation.
     * The second byte is used by the operation. A register requires a plugin type and a read or write requires an id.
     * The rest of the payload gets passed to the plugin to handle.
     */

    enum class MessageHeader
    {
        Register = 0,
        Write = 1,
        Read = 2,
    };

    enum class PluginType
    {
        None = 0, // Used as a fail type
        PCA9685 = 1,
        VL6180X = 2,
    };

    static constexpr unsigned int MaxAddressablePlugins = 10;

    struct Registration {
        unsigned int id = 0;
        bool isSuccesful = false;
    };

    class PluginManager {
        public:
            PluginManager();
            ~PluginManager();
            Registration RegisterPlugin(byte argc, byte *argv);
            void HandleMessage(byte argc, byte *argv);
            IPlugin* GetPlugin(unsigned int id);

        private:
            IPlugin* m_plugins[MaxAddressablePlugins] = { nullptr };
            unsigned int m_sizeOfPlugins = 0;
    };
}