//
// Shim to wrap MAVSDK c++ api
// Specifically adds constructor functions to allow passing through ffi
//
// Also houses Helper functions to assist with c++ specific functionality
//

#ifndef FLIGHTCTL_SRC_INCLUDE_HELPER_H_
#define FLIGHTCTL_SRC_INCLUDE_HELPER_H_

#include <mavsdk/mavsdk.h>
#include <mavsdk/plugins/telemetry/telemetry.h>
#include <mavsdk/plugins/action/action.h>
#include <mavsdk/plugins/mission/mission.h>
#include <mavsdk/plugins/mission_raw/mission_raw.h>

// Callback code
extern "C" {
typedef void (*rust_cb_log)(void *, char const *);
typedef void (*rust_cb_run_position)(void *, double_t, double_t, float_t);
typedef void (*rust_cb_run_progress)(void *, int32_t, int32_t);
typedef void (*rust_cb_run_complete)(void *);
};

// Shim code
extern "C" {
typedef mavsdk::Mavsdk *SDKHandle;
typedef mavsdk::System *SHandle;
SDKHandle new_mavsdk();
void del_mavsdk(SDKHandle);
};

// Helper functions
extern "C" {
typedef std::vector<mavsdk::MissionRaw::MissionItem> *MRHandle;
SHandle connect(SDKHandle, char const *);
int32_t run_qgc_plan(SHandle,
                     char const *,
                     void *,
                     rust_cb_run_position,
                     rust_cb_run_progress,
                     rust_cb_run_complete,
                     rust_cb_log);
};

// String format function
template<typename ... Args>
std::string string_format(const std::string &format, Args ... args) {
  int size_s = std::snprintf(nullptr, 0, format.c_str(), args ...) + 1;
  if (size_s <= 0) { throw std::runtime_error("Error during formatting."); }
  auto size = static_cast<size_t>( size_s );
  std::unique_ptr<char[]> buf(new char[size]);
  std::snprintf(buf.get(), size, format.c_str(), args ...);
  return std::string(buf.get(), buf.get() + size - 1);
}

#endif //FLIGHTCTL_SRC_INCLUDE_HELPER_H_
