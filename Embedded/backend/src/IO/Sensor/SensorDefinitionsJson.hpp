#pragma once

#include <ArduinoJson.h>
#include "SensorDefinitions.hpp"

namespace IO {
   SensorTemplate JsonToSensorTemplate(const JsonVariant& json);
}
