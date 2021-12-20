#pragma once

namespace Comms {
    enum class CommType {
        Driver,
        Passenger
    };

    class i2c {
    public:
        i2c();
        i2c(int address);
        void Connect();

    private:
        int m_address;
        CommType m_commType;
    };
}