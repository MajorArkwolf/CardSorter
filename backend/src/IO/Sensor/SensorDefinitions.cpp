#include "SensorDefinitions.hpp"
#include "../../BoardDefinitions.hpp"

namespace IO {
    SensorDataTypes::SensorDataTypes() {
        memset( this, 0, sizeof( SensorDataTypes ) );
    }

    SensorDataTypes::SensorDataTypes(const Shared::Color& newColor) {
        this->color = newColor;
    }

    SensorDataTypes::SensorDataTypes(bool newBoolean) {
        this->boolean = newBoolean;
    }

    SensorDataTypes::SensorDataTypes(int8_t newIntegar) {
        this->integar = newIntegar;
    }
}
