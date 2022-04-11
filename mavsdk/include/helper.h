//
// Houses Helper functions to assist with c++ specific functionality
//

#ifndef FLIGHTCTL_MAVSDK_INCLUDE_HELPER_H_
#define FLIGHTCTL_MAVSDK_INCLUDE_HELPER_H_

#include <memory>

#include "shim.h"
#include "rust/cxx.h"

namespace mavsdk {

struct FMissionItem;

std::shared_ptr<System> connect(Mavsdk &sdk, rust::String addr);

}

#endif //FLIGHTCTL_MAVSDK_INCLUDE_HELPER_H_
