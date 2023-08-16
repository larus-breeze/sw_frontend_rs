from flight_data import FlightData
import time
import keyboard
import os

KEY_F12 = 88
KEY_UP = 72
KEY_DOWN = 80
KEY_LEFT = 75
KEY_Right = 77

class FlightPlayer():
    def __init__(self, file_name: str):
        print(f"load file '{file_name}'", end='', flush=True)
        self._data = FlightData(file_name, 5005)
        print(" finished")
        self._next_wakeup_time = 0
        keyboard.on_press(self.on_key_press)
        self._kbd_active = False

    def set_offset(self, seconds: int):
        self._data.set_offset(seconds)

    def run(self):
        self._next_wakeup_time = time.time() + 0.1
        self._data.set_offset(60 * 150)
        try:
            while True:
                if time.time() < self._next_wakeup_time:
                    time.sleep(0.005)  # we sleep a bit while waiting to not block the thread
                else:
                    self._data.tick()
                    self._data.can_send_frames()
                    print(f"Flight time {self._data.time()}", end='\r', flush=True)
                    self._next_wakeup_time += 0.1
        except KeyboardInterrupt:
            print('\nThe flight player stopped by the user')

    def on_key_press(self, key_event: keyboard.KeyboardEvent):
        if self._kbd_active:
            print(key_event.scan_code)
            match key_event.scan_code:
                case 77: self._data.inc_time(60)    # key right
                case 75: self._data.inc_time(-60)   # key left
                case 72: self._data.inc_time(3600)  # key up
                case 80: self._data.inc_time(-3600) # key down
                case _: pass

        if key_event.scan_code == KEY_F12:
            self._kbd_active = not self._kbd_active
            print('Flightplayer Command active: ', self._kbd_active)



fp = FlightPlayer("hm" + os.sep + "230617_091240.f37.f110")
fp.run()
