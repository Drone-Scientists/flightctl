//
// Shim to wrap MAVSDK c++ api
// Specifically adds constructor functions to allow passing through ffi
//

#ifndef FLIGHTCTL_SRC_INCLUDE_SHIM_H_
#define FLIGHTCTL_SRC_INCLUDE_SHIM_H_

#include <mavsdk/mavsdk.h>
#include <mavsdk/plugins/telemetry/telemetry.h>
#include <mavsdk/plugins/action/action.h>

extern "C" {
typedef mavsdk::Mavsdk *MHandle;
typedef mavsdk::System *SHandle;
MHandle new_mavsdk();
void del_mavsdk(MHandle);
};

#endif //FLIGHTCTL_SRC_INCLUDE_SHIM_H_
