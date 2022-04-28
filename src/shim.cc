#include "include/shim.h"

extern "C" {
  MHandle new_mavsdk() {
    return new mavsdk::Mavsdk;
  }
  void del_mavsdk(MHandle p) {
    delete p;
  }
}
