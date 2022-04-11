//
// Shim to wrap MAVSDK c++ api
// Specifically adds constructor functions to allow passing through ffi
//

#ifndef FLIGHTCTL_MAVSDK_INCLUDE_PLUGINS_H_
#define FLIGHTCTL_MAVSDK_INCLUDE_PLUGINS_H_

#include <mavsdk/mavsdk.h>
#include <mavsdk/plugins/telemetry/telemetry.h>
#include <mavsdk/plugins/action/action.h>


namespace mavsdk {

std::unique_ptr<Mavsdk> new_mavsdk();
std::unique_ptr<Telemetry> new_telemetry(std::shared_ptr<System> sys);

}

#endif //FLIGHTCTL_MAVSDK_INCLUDE_PLUGINS_H_
