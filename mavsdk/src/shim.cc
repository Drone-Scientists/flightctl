#include "../include/shim.h"

namespace mavsdk {

std::unique_ptr<Mavsdk> new_mavsdk() {
  return std::make_unique<Mavsdk>();
}
std::unique_ptr<Telemetry> new_telemetry(std::shared_ptr<System> sys) {
  return std::make_unique<Telemetry>(sys);
}

}