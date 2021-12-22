#pragma once
#include "../Comms/IComm.hpp"
#include "SensorMessage.hpp"

namespace IO { 
    class NetworkSensorInterface {
    public:
        NetworkSensorInterface(int m_address, Comm::IComm* comm);
        SensorMessageResponse SendAndRecieveMessage(const SensorMessage& message);
    private:
        Comm::IComm* m_comm;
        int m_address;
    };
}