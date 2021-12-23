#pragma once

namespace JSON {
    enum JsonKeys {
        Create,
        Update,
        Reset,
        Error,
    };

    JsonKeys StringToEnum(const char* string) {
        if (string == "Create") { return Create; }
        if (string == "Update") { return Update; }
        if (string == "Reset") { return Reset; }
        return Error; 
    }
}
