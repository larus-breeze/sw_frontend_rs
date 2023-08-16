from dataformats import *
from utils import *

from math import atan2, pi, sqrt
from datetime import date, time
import numpy
import pandas
import socket


class FlightData():
    def __init__(self, udp_port: int, file_name: str|None=None):
        self._idx = 0
        self._last_idx = 0
        self._delta = 10

        self._ip = '127.0.0.1'
        self._port = udp_port
        self._socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)  # Internet, UDP

        self._df = None
        self._row = None
        if file_name is not None:
            self.from_file(file_name)

    def from_file(self, file_name):
        if file_name.endswith('.f37'):
            dataformat = data_f37
        elif file_name.endswith('.f50'):
            dataformat = data_f50
        elif file_name.endswith('.f123'):
            dataformat = data_f123
        elif file_name.endswith('.f120'):
            dataformat = data_f120
        elif file_name.endswith('.f110'):
            dataformat = data_f110
        else:
            raise NotImplementedError("Format not supported")

        # Create a pandas dataframe
        format = numpy.dtype(dataformat)
        data = numpy.fromfile(file_name, dtype=format, sep="")

        # store references in class instance
        self._df = pandas.DataFrame(data)
        self._last_idx = len(self._df.index) - 1
        self._idx = 0
        self._row = self._df.iloc[self._idx]

    def set_offset(self, seconds: int):
        self._idx = seconds * 100
        self._check_idx_range()

    def set_relative(self, pos: int): # 0..999
        self._idx = int(pos * 0.001 * self._last_idx)
        self._row = self._df.iloc[self._idx]

    def inc_time(self, seconds):
        self._idx += seconds*100
        self._check_idx_range()

    def _check_idx_range(self):
        if self._idx > self._last_idx:
            self._idx = self._last_idx
        elif self._idx < 0:
            self._idx = 0

    def tick(self):
        self._idx += self._delta
        self._check_idx_range()
        self._row = self._df.iloc[self._idx]

    def set_speed(self, speed):
        if speed < 0.1:
            speed = 0.1
        elif speed > 100.0:
            speed = 100.0
        self._delta = int(10 * speed)

    def date_of_flight(self) -> date:
        row = self._df.iloc[self._last_idx]
        year = int(row['year']) + 2000
        month = int(row['month'])
        day = int(row['day'])
        return date(year, month, day)

    def time(self) -> time:
        try:
            hour = int(self._row['hour'])
            min = int(self._row['minute'])
            sec = int(self._row['second'])
            return time(hour, min, sec)
        except:
            return time(0, 0, 0)

    def start_recording(self) -> time:
        try:
            row = self._df.iloc[2000]     # 20 sec after power on, GPS has usually a fix
            hour = int(row['hour'])
            min = int(row['minute'])
            sec = int(row['second'])
            return time(hour, min, sec)
        except:
            return time(0, 0, 0)

    def end_recording(self) -> time:
        try:
            row = self._df.iloc[self._last_idx]
            hour = int(row['hour'])
            min = int(row['minute'])
            sec = int(row['second'])
            return time(hour, min, sec)
        except:
            return time(0, 0, 0)

    def __getitem__(self, item: str) -> float:
        return self._row[item]

    def can_send_frames(self):
        try:
            self._can_send_frames()
        except:
            pass # silently ignore errors

    def _can_send_frames(self):

        # AIRSPEED tas, ias in km/h
        self.can_send(to_u16(0x0102) +
                      to_i16(self._row['TAS'] * 3.6) +
                      to_i16(self._row['IAS'] * 3.6))

        # VARIO climb_rate, average_climb_rate in mm/s
        self.can_send(to_u16(0x103) +
                      to_i16(self._row['vario'] * 1000.0) +
                      to_i16(self._row['vario integrator'] * 1000.0))

        # WIND wind 0.001 rad i16 km/h i16 avg wind 0.001 rad i16 km/h i16
        wind_direction = atan2(- self._row['wind E'], - self._row['wind N'])
        if (wind_direction < 0.0):
            wind_direction += 2 * pi
        av_wind_direction = atan2(- self._row['wind avg E'], - self._row['wind avg N'])
        if (av_wind_direction < 0.0):
            av_wind_direction += 2 * pi
        self.can_send(to_u16(0x108) +
                      to_i16(wind_direction * 1000.0) +
                      to_i16(sqrt(self._row['wind E'] ** 2 + self._row['wind N'] ** 2) * 3.6) + \
                      to_i16(av_wind_direction * 1000.0) +
                      to_i16(sqrt(self._row['wind avg E'] ** 2 + self._row['wind avg N'] ** 2) * 3.6))

        # ATHMOSPHERE static pressure in Pa, air_density in g/m³
        self.can_send(to_u16(0x109) +
                      to_u32(self._row['Pressure-altitude']) +
                      to_u32(self._row['Air Density'] * 1000.0))

        # ACCELERATION g_load in mm/s², eff_vert_acc mm/s², vario_uncomp in mm/s,
        #              circle_mode (0 straigt, 1 transition, 2 circling)
        self.can_send(to_u16(0x10b) +
                      to_i16(self._row['G_load'] * 1000.0) +
                      to_i16(self._row['acc vertical'] * -1000.0) +
                      to_i16(self._row['vario uncomp'] * -1000.0) +
                      to_u8(self._row['circle mode']))

        # TURN_COORD slip_angle in rad, turn_rate in rad/s, nick_angle in rad
        self.can_send(to_u16(0x10c) + \
                      to_i16(self._row['slip angle'] * 1000.0) +
                      to_i16(self._row['turn rate'] * 1000.0) +
                      to_i16(self._row['nick angle'] * 1000.0))

    def can_send(self, frame: bytes):
        self._socket.sendto(frame, (self._ip, self._port))

