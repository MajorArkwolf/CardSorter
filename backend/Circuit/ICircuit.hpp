#pragma once

namespace Circuit {
    class ICircuit {
    public:
        virtual ~ICircuit(){}
        virtual bool IsReady() = 0;
        virtual bool IsComplete() = 0;
    };
}