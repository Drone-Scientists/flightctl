#include <cstdint>
#include <iostream>
#include <thread>
#include <chrono>
#include <functional>
#include <future>
#include <memory>
#include <string>
#include <ctime>
#include <fstream>
#include "include/helper.h"

using std::this_thread::sleep_for;
using std::chrono::seconds;

// shim code
extern "C" {
SDKHandle new_mavsdk() {
  return new mavsdk::Mavsdk;
}
void del_mavsdk(SDKHandle p) {
  delete p;
}
}

// helper code
extern "C" {
SHandle connect(SDKHandle sdk, char const *addr) {
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

int32_t run_qgc_plan(SHandle system,
                     char const *path,
                     void *rust_cb,
                     rust_cb_run_position cb_pos,
                     rust_cb_run_progress cb_pro,
                     rust_cb_run_complete cb_com,
                     rust_cb_log cb_log) {
  // setup plugins
  auto telemetry = mavsdk::Telemetry{*system};
  auto action = mavsdk::Action{*system};
  auto mission_raw = mavsdk::MissionRaw(*system);

  const mavsdk::Telemetry::Result set_rate_result = telemetry.set_rate_position(1.0);
  if (set_rate_result != mavsdk::Telemetry::Result::Success) {
    cb_log(rust_cb, "Failed to connect to system");
    return -1;
  }

  cb_log(rust_cb, "Setting up Position monitoring");
  telemetry.subscribe_position([&](mavsdk::Telemetry::Position position) {
    cb_pos(rust_cb, position.latitude_deg, position.longitude_deg, position.relative_altitude_m);
  });

  // check for vehicle to be ready to arm
  while (telemetry.health_all_ok()) {
    cb_log(rust_cb, "Waiting for vehicle to arm");
    sleep_for(seconds(1));
  }

  cb_log(rust_cb, "Pulling mission data from plan file");
  auto import_plan = mission_raw.import_qgroundcontrol_mission(path);
  if (import_plan.first != mavsdk::MissionRaw::Result::Success) {
    cb_log(rust_cb, string_format("Failed to import mission: {}", import_plan.first).c_str());
    return -1;
  }
  if (import_plan.second.mission_items.empty()) {
    cb_log(rust_cb, "Mission is empty");
    return -1;
  }

  cb_log(rust_cb, "Uploading mission to system");
  const auto upload_result = mission_raw.upload_mission(import_plan.second.mission_items);
  if (upload_result != mavsdk::MissionRaw::Result::Success) {
    cb_log(rust_cb, string_format("Failed to upload mission to system: {}", upload_result).c_str());
    return -1;
  }
  cb_log(rust_cb, "Successfully uploaded mission");

  cb_log(rust_cb, "Arming system");
  const auto arm_result = action.arm();
  if (arm_result != mavsdk::Action::Result::Success) {
    cb_log(rust_cb, string_format("Arm Failed: {}", arm_result).c_str());
  }
  cb_log(rust_cb, "Arming complete");

  mission_raw.subscribe_mission_progress([&](mavsdk::MissionRaw::MissionProgress mission_progress) {
    cb_pro(rust_cb, mission_progress.current, mission_progress.total);
  });

  cb_log(rust_cb, "Starting Mission");
  const auto start_result = mission_raw.start_mission();
  if (start_result != mavsdk::MissionRaw::Result::Success) {
    cb_log(rust_cb, string_format("Mission start failed: {}", start_result).c_str());
    return -1;
  }
  cb_com(rust_cb);
  return 0;
}
}
