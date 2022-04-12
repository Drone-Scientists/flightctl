#include <future>
#include <chrono>
#include <iostream>
#include "../include/shim.h"
#include "flightctl/src/lib.rs.h"

namespace mavsdk {

std::shared_ptr<System> connect(Mavsdk &sdk, rust::String addr) {
  ConnectionResult connection_result = sdk.add_any_connection(std::string(addr));
  if (connection_result != ConnectionResult::Success) {
    std::cerr << "Connection failed: " << connection_result << '\n';
    return std::shared_ptr<System>(nullptr);
  }

  std::cout << "Waiting to discover system... \n";
  auto prom = std::promise < std::shared_ptr < System >> {};
  auto fut = prom.get_future();

  sdk.subscribe_on_new_system([&sdk, &prom]() {
    auto system = sdk.systems().back();

    if (system->has_autopilot()) {
      std::cout << "Discovered autopilot\n";
      sdk.subscribe_on_new_system(nullptr);
      prom.set_value(system);
    }
  });

  if (fut.wait_for(std::chrono::seconds(3)) == std::future_status::timeout) {
    std::cerr << "No autopilot found.\n";
    return std::shared_ptr<System>(nullptr);
  }

  return fut.get();
}

}