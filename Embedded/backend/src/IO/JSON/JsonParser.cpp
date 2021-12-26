#include "JsonParser.hpp"
namespace JSON {
    IO::SensorInitData JsonToSensorInitData(IO::Definition::SensorType Type, const JsonObject& json) {
        auto initData = IO::SensorInitData();
        switch(Type) {
            case IO::Definition::SensorType::Motor:
            case IO::Definition::SensorType::ServoMotor:
            case IO::Definition::SensorType::DeattachedServoMotor:
                initData.motorData.Pin = json["Pin"];
                break;
            case IO::Definition::SensorType::PhotoResistor:
                initData.photoResitorData.Pin = json["Pin"];
                break;
            case IO::Definition::SensorType::PixelLight:
                initData.pixelLightData.Pin = json["Pin"];
                initData.pixelLightData.NumberOfPixels = json["NumberOfPins"];
                break;
            default:
                break;
        }
        return initData;
    }

    IO::Method JsonToSensorMethod(IO::Definition::SensorType Type, const JsonObject& json) {
        auto method = IO::Method();
        const auto methodID = json["Method"].as<int>();
        switch (Type)
        {
            case IO::Definition::SensorType::Motor:
                method.motorMethods = (IO::MotorMethods)methodID;
                break;
            case IO::Definition::SensorType::ServoMotor:
            case IO::Definition::SensorType::DeattachedServoMotor:
                method.servoMotorMethod = (IO::ServoMotorMethods)methodID;
                break;
            case IO::Definition::SensorType::PhotoResistor:
                method.photoResistorMethod= (IO::PhotoResistorMethods)methodID;
                break;
            case IO::Definition::SensorType::PixelLight:
                method.pixelLightMethod = (IO::PixelLightMethods)methodID;
                break;
            default:
                break;
        }
    }

    IO::SensorDataTypes JsonToSensorData(const JsonObject& json) {
        auto data = IO::SensorDataTypes();

        return data;
    }

    IO::SensorTemplate JsonToSensorTemplate(const JsonObject& json) {
        auto message = IO::SensorTemplate();
        message.boardAddress = json["Board"].as<int>();
        message.type = (IO::Definition::SensorType)json["Type"].as<int>();
        message.data = JsonToSensorInitData(message.type, json["params"]);
        return message;
    }

    IO::SensorMessage JsonToSensorMessage(const JsonObject& json) {
        auto message = IO::SensorMessage();
        message.sensorID = json["ID"];
        message.type = (IO::Definition::SensorType)json["Type"].as<int>();
        message.method = JsonToSensorMethod(message.type, json);
        message.data = JsonToSensorData(json);
        return message;
}
}