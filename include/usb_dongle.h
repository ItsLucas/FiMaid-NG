#include <mutex>
#define USB_PORT "1-1"

/* Singleton usb_dongle class, controls poweron and poweroff. */
/* Power-on command: uhubctl -l USB_PORT -a 1 */
/* Power-off command: uhubctl -l USB_PORT -a 0 */
/* @see usb_dongle.cc */
class usb_dongle {
public:
    static usb_dongle& instance() {
        static usb_dongle instance;
        return instance;
    }

    void poweron();
    void poweroff();
    bool status() const {
        return m_status;
    }
  private:
    usb_dongle() = default;
    ~usb_dongle() = default;
    usb_dongle(const usb_dongle&) = delete;
    usb_dongle &operator=(const usb_dongle &) = delete;

    std::mutex mutex;
    bool m_status{false};
};