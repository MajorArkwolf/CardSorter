#include "PluginManager.h"
#include "PCA9685.h"
#include "VL6180X.h"

namespace Ark {

    PluginManager::PluginManager() {

    }

    PluginManager::~PluginManager() {
        for (auto index = 0; index < m_sizeOfPlugins; ++index) {
            delete(m_plugins[index]);
        }
    }

    Registration PluginManager::RegisterPlugin(byte argc, byte *argv) {
        auto registration = Registration();
        if (m_sizeOfPlugins >= MaxAddressablePlugins) {
            // Cant register a new plugin if we are out of space.
            return registration;
        }

        // Get plugin type and the message to be passed into the plugin.
        auto pluginType = *reinterpret_cast<PluginType*>(argv[0]);
        auto pluginMessage = &argv[1];

        // Serves as our temporary storage before we move it into our storage container.
        IPlugin *plugin = nullptr;

        // Pass registration onto the plugin.
        switch (pluginType) {
            case PluginType::PCA9685:
                plugin = new PCA9685(argc - 1, pluginMessage);
                registration.isSuccesful = true;
                break;
            case PluginType::VL6180X:
                plugin = new VL6180X(argc - 1, pluginMessage);
                registration.isSuccesful = true;
                break;
            case PluginType::None:
            default:
                break;
        };

        // If succesfully found and allocated update our registration message
        // and increment our storage counter.
        if (registration.isSuccesful && plugin != nullptr) {
            m_plugins[m_sizeOfPlugins] = plugin;
            registration.id = m_sizeOfPlugins;
            ++m_sizeOfPlugins;
        } else {
            delete plugin;
            registration.isSuccesful = false;
        }

        return registration;
    }

    IPlugin* PluginManager::GetPlugin(unsigned int id) {
        if (id < m_sizeOfPlugins) {
            return m_plugins[id];
        } else {
            return nullptr;
        }
    }

    void PluginManager::HandleMessage(byte argc, byte *argv) {
        // Need atleast 2 argument to be able to do anything.
        if (argc < 2 || argv == nullptr) {
            return;
        }

        auto header = static_cast<MessageHeader>(argv[0]);
        if (header == MessageHeader::Register) {
            auto type = static_cast<PluginType>(argv[1]);
        } else if (header == MessageHeader::Write) {
            
        } else if (header == MessageHeader::Read) {

        }
    }
}