#pragma once

namespace Operation {
    enum class Result {
        Success = 0,
        Failure = -1,
        NoOp = 1,
        CriticalFailure = -2
    };
}