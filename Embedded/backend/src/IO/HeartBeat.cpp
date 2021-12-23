#include "HeartBeat.hpp"
#include "Sensor/Sensor.hpp"

namespace System {

    HeartBeat::HeartBeat(int boardAddress, System::BoardStatus systemStatus) : 
    m_myAddress(boardAddress),
    m_systemStatus(systemStatus) {}

    System::BoardStatus HeartBeat::GetBoardStatus() const {
        return m_systemStatus;
    }
    int HeartBeat::GetAddress() const {
        return m_myAddress;
    }

    size_t HeartBeat::GetNumberOfIDs() const {
        return m_sensorIDs.GetSize();
    }

    const Container::Array<IO::SensorID>& HeartBeat::GetSensorIDs() const {
        return m_sensorIDs;
    }

    void HeartBeat::PopulateIDList(const Container::Array<IO::Sensor*>& sensorList) {
        for (auto i = 0; i < sensorList.GetSize(); ++i) {
            IO::SensorID id = sensorList.Get(i)->GetID();
            m_sensorIDs.Append(id);
        }
    }
}