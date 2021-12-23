#include "IOOverseer.hpp"
#include "IOFactory.hpp"
#include "JSON/JsonDefinitions.hpp"
#include "JSON/JsonParser.hpp"
#include "Sensor/SensorDefinitions.hpp"


namespace IO {
    IOOverseer::IOOverseer() : m_sensorIDDistributor(),
    m_sensorMapping(),
    m_systemStatus(System::BoardStatus::WaitingSetup),
    m_i2c(),
    m_serialComm() {}
    
    IOOverseer::~IOOverseer() {}

    void IOOverseer::Setup() {
        m_i2c.Connect();

    }

    void IOOverseer::Update() {
        if (m_serialComm.SerialDataPending()) {
            auto jsonContents = m_serialComm.Recieve().to<JsonObject>();
            for (auto i = jsonContents.begin(); i != jsonContents.end(); ++i) {
                if (JSON::StringToEnum(i->key().c_str()) == JSON::JsonKeys::Create) {
                    RegisterSensor(JSON::JsonToSensorTemplate(i->value()));
                }
                if (JSON::StringToEnum(i->key().c_str()) == JSON::JsonKeys::Update) {

                }
                if (JSON::StringToEnum(i->key().c_str()) == JSON::JsonKeys::Reset) {
                    break;
                }
                if (JSON::StringToEnum(i->key().c_str()) == JSON::JsonKeys::Error) {

                }
            }
        }
    }

    void IOOverseer::RegisterSensor(const SensorTemplate& data) {
        if(data.boardAddress != 0) {
            SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
            auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, data.type, newId, data.data);
            m_i2c.Send(data.boardAddress, MessageProtocol::Message(MessageProtocol::MessageType::CreateSensor, MessageProtocol::GenericMessageToBytes(sensorInfo)).MessageToBytes());
            auto response = MessageProtocol::Message::BytesToMessage(m_i2c.Recieve());
            if (response.GetMessageType() == MessageProtocol::MessageType::Acknowledge) {
                auto factory = Factory::IOFactory(&m_i2c);
                auto networkData = SensorInitData(NSensor(data.boardAddress));
                sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Network, data.type, newId, networkData);
                m_sensorMapping.Append(factory.CreateSensor(sensorInfo));
            }
        } else {
            SensorID newId = m_sensorIDDistributor.GetNewUniqueID();
            auto factory = Factory::IOFactory(&m_i2c);
            auto sensorInfo = Factory::FactoryMessage(Factory::SensorLocation::Local, data.type, newId, data.data);
            auto sensor = factory.CreateSensor(sensorInfo);
            sensor->Setup();
            m_sensorMapping.Append(sensor);
        }
    }
}