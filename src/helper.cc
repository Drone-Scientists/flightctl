#include <future>
#include <chrono>
#include <iostream>
#include "include/helper.h"

extern "C" {
SHandle connect(MHandle sdk, char const *addr) {
  mavsdk::ConnectionResult connection_result = sdk->add_any_connection(std::string(addr));
  if (connection_result != mavsdk::ConnectionResult::Success) {
    std::cerr << "Connection failed: " << connection_result << '\n';
    return nullptr;
  }

  std::cout << "Waiting to discover system... \n";
  auto prom = std::promise < std::shared_ptr < mavsdk::System >> {};
  auto fut = prom.get_future();

  sdk->subscribe_on_new_system([&sdk, &prom]() {
    auto system = sdk->systems().back();

    if (system->has_autopilot()) {
      std::cout << "Discovered autopilot\n";
      sdk->subscribe_on_new_system(nullptr);
      prom.set_value(system);
    }
  });

  if (fut.wait_for(std::chrono::seconds(3)) == std::future_status::timeout) {
    std::cerr << "No autopilot found.\n";
    return nullptr;
  }

  return fut.get().get();
}
}

