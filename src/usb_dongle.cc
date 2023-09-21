#include "usb_dongle.h"

#include <mutex>
#include <string>
#include <cstdlib>

void usb_dongle::poweron() {
  std::lock_guard<std::mutex> lock(mutex);
  std::string command = "uhubctl -l " + std::string(USB_PORT) + " -a 1";
  std::system(command.c_str());
}

void usb_dongle::poweroff() {
  std::lock_guard<std::mutex> lock(mutex);
  std::string command = "uhubctl -l " + std::string(USB_PORT) + " -a 0";
  std::system(command.c_str());
}