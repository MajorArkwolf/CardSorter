#include "SensorMessage.hpp"


namespace IO {
    Method::Method() { 
        memset( this, 0, sizeof( Method ) ); 
    }

    Method::Method(ServoMotorMethods methodInvoking) {
        this->servoMotorMethod = methodInvoking;
    }

    Method::Method(PixelLightMethods methodInvoking) {
        this->pixelLightMethod = methodInvoking;
    }

    Method::Method(PhotoResistorMethods methodInvoking) {
        this->photoResistorMethod = methodInvoking;
    }

    SensorMessage::SensorMessage() {
        sensorID = 0;
        type = Definition::SensorType::None;
        method = Method();
        data = SensorDataTypes();
    }

    SensorMessage::SensorMessage(int id, Definition::SensorType destType, Method methodInvoking, SensorDataTypes dataRequired) :
    sensorID(id),
    type(destType),
    method(methodInvoking),
    data(dataRequired)
    {}

    SensorMessageResponse::SensorMessageResponse() {
        wasSuccessful = false;
        data = SensorDataTypes();
    }

    SensorMessageResponse::SensorMessageResponse(bool successful, SensorDataTypes dataIn) : 
    wasSuccessful(successful), 
    data(dataIn) {}
}