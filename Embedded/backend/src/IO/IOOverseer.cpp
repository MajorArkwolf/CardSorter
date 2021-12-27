#include "IOOverseer.hpp"
#include "IOFactory.hpp"
#include "JSON/JsonDefinitions.hpp"
#include "JSON/JsonParser.hpp"
#include "Sensor/SensorDefinitions.hpp"


namespace IO {
    IOOverseer::IOOverseer() : m_sensorIDDistributor(),
    m_sensorMapping(),
    m_i2c(),
    m_serialComm(),
    m_boardAddresses(),
    m_systemStatus(System::BoardStatus::WaitingSetup) {}
    
    IOOverseer::~IOOverseer() {}

    void IOOverseer::Setup() {
        m_i2c.Connect();
        m_serialComm.Connect();
    }

    void IOOverseer::Update() {
        HandleSerialMessage();
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

    void IOOverseer::HandleSerialMessage() {
        if (m_serialComm.SerialDataPending()) {
            auto doc = DynamicJsonDocument(30);
            auto isValid = m_serialComm.Recieve(doc);
            if (isValid == true) {
                if (!doc["Ping"].isNull()) {
                    char pong[] = "{\"Pong\": true}";
                    auto doc = DynamicJsonDocument(20);
                    DeserializationError error = deserializeJson(doc, pong);
                    m_serialComm.Send(doc);
                    return;
                } else if (!doc["Register"].isNull()) {
                    auto jsonRegistration = doc["Register"].as<JsonVariant>();
                    HandleSerialSetupMessage(jsonRegistration);
                    return;
                } else if (!doc["Update"].isNull()) {
                    if (m_systemStatus == System::BoardStatus::Running) {
                        auto jsonSensorArray = doc["Update"].as<JsonArrayConst>();
                    }
                } else if (!doc["Reset"].isNull()) {

                } else {
                    m_serialComm.Send(String("Error"), "Unknown");
                }
            }
            m_serialComm.Send(String("Error"), "Unexpected end of IOOverseer::HandleSerialMessage");
        }
    }

    void IOOverseer::HandleSerialSetupMessage(const JsonVariant& jsonReg) {
        if(jsonReg.containsKey("Board")) {
            // setup all boards
            if (jsonReg.containsKey("Sensors")) {
                m_serialComm.Send("Register", "Sensors");
            }
            m_serialComm.Send("Register", "board");
            return;
        }
        m_serialComm.Send("Register", "fell thro while also being a stupidly long message to see if the payload is being fucked hard");
    }
}