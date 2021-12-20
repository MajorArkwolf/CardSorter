#include "SensorMessage.hpp"


namespace IO {
    Method::Method() { 
        memset( this, 0, sizeof( Method ) ); 
    }

    SensorMessage::SensorMessage() {
        sensorID = 0;
        type = Definition::SensorType::None;
        method = Method();
        data = SensorDataTypes();
    }

    SensorMessageResponse::SensorMessageResponse() {
        wasSuccessful = false;
        data = SensorDataTypes();
    }
}