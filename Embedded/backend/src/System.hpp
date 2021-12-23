#pragma once

namespace System {
    enum class BoardStatus {
        Failure = 0,
        WaitingSetup,
        Running,
        Stopped,
    };
}