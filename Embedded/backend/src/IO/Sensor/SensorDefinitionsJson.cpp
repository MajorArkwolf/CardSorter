#include "SensorDefinitionsJson.hpp"

namespace {
    IO::SensorInitData ParseData(IO::Definition::SensorType type, const JsonVariant& json) {
        auto dataOut = IO::SensorInitData();
        switch (type)
        {
        case IO::Definition::SensorType::PixelLight:
            dataOut.pixelLightData.Pin = json["Pin"];
            dataOut.pixelLightData.NumberOfPixels = json["NumberOfPixels"];
            break;
        case IO::Definition::SensorType::PhotoResistor:
            dataOut.photoResitorData.Pin = json["Pin"];
            break;
        case IO::Definition::SensorType::ServoMotor:
        case IO::Definition::SensorType::DeattachedServoMotor:
        case IO::Definition::SensorType::Motor:
            dataOut.motorData.Pin = json["Pin"];
            break;
        default:
            break;
        }
        return dataOut;
    }
}

namespace IO {
   SensorTemplate JsonToSensorTemplate(const JsonVariant& json) {
        auto temp = SensorTemplate();
        temp.boardAddress = json["BoardAddress"];
        temp.type = json["SensorType"];
        temp.data = ParseData(temp.type, json["param"]);
        return temp;
    }
}
