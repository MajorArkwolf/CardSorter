#include "../Comms/IComm.hpp"
#include "SensorMessage.hpp"

namespace IO { 
    class NetworkSensorInterface {
    public:
        NetworkSensorInterface();
        NetworkSensorInterface(Comm::IComm* comm);
        SensorMessageResponse SendAndRecieveMessage(int address, const SensorMessage& message);
    private:
        Comm::IComm* m_comm;
    };
}