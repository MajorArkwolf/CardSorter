#pragma once
#include "../Array.hpp"
#include "Sensor/SensorDefinitions.hpp"
#include "../System.hpp"

namespace IO {
    class Sensor;
}

namespace System {
    class HeartBeat {
    public:
        HeartBeat(int boardAddress, System::BoardStatus systemStatus);
        void PopulateIDList(const Container::Array<IO::Sensor*>& sensorList);
        int GetAddress() const;
        System::BoardStatus GetBoardStatus() const;
        size_t GetNumberOfIDs() const;
        const Container::Array<IO::SensorID>& GetSensorIDs() const;

    private:
        Container::Array<IO::SensorID> m_sensorIDs;
        int m_myAddress;
        System::BoardStatus m_systemStatus;
    };
}