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

    SensorDataTypes::SensorDataTypes(int newIntegar) {
        this->integar = newIntegar;
    }

    SensorInitData::SensorInitData() { 
        memset( this, 0, sizeof( SensorInitData ) ); 
    }

    SensorInitData::SensorInitData(NSensor data) {
        this->networkSensor = data;
    }
    SensorInitData::SensorInitData(PhotoResitorData data) {
        this->photoResitorData = data;
    }
    SensorInitData::SensorInitData(ServoMotorData data) {
        this->servoMotorData = data;
    }
    SensorInitData::SensorInitData(PixelLightData data) {
        this->pixelLightData = data;
    }

}
