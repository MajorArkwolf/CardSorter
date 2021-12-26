#pragma once
#include "../../Message.hpp"
#include <ArduinoJson.h>
#include "../Sensor/SensorDefinitions.hpp"
#include "../Sensor/SensorMessage.hpp"

namespace JSON {
    // Main Messages
    IO::SensorTemplate JsonToSensorTemplate(const JsonObject& json);
    IO::SensorMessage JsonToSensorMessage(const JsonObject& json);

    // Helper Methods
    IO::SensorInitData JsonToSensorInitData(IO::Definition::SensorType Type, const JsonObject& json);
    IO::Method JsonToSensorMethod(IO::Definition::SensorType Type, const JsonObject& json);
    IO::SensorDataTypes JsonToSensorData(const JsonObject& json);
}