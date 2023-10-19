from datetime import date, time
import numpy
import pandas

from dataformats import *

# This class provides access to the flight data. To realize this, dataframes from the Pandas 
# project are used. The access is still supplemented by some comfort functions

class FlightData():
    def __init__(self, file_name: str|None=None):
        """Initializes the FlightData class"""
        self._idx = 0
        self._last_idx = 0
        self._delta = 10

        self._df = None
        self._row = None
        if file_name is not None:
            self.from_file(file_name)

    def from_file(self, file_name):
        """Opens a Larus flight data file"""
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
        """Sets the time depending on the start point"""
        self._idx = seconds * 100
        self._check_idx_range()
        self._row = self._df.iloc[self._idx]

    def set_relative(self, pos: int): # 0..999
        """Sets the time in scale 0..999"""
        self._idx = int(pos * 0.001 * self._last_idx)
        self._check_idx_range()
        self._row = self._df.iloc[self._idx]

    def get_relative(self) -> int:
        """Returns the relative position"""
        return round((999.0 * self._idx) / self._last_idx)

    def tick(self):
        """Selects the next data set (default is plus 0.1 seconds)."""
        self._idx += self._delta
        self._check_idx_range()
        self._row = self._df.iloc[self._idx]

    def set_speed(self, speed):
        """Sets the playback speed"""
        if speed < 0.1:
            speed = 0.1
        elif speed > 100.0:
            speed = 100.0
        self._delta = int(10 * speed)

    def date_of_flight(self) -> date:
        """Returns the date of the flight"""
        row = self._df.iloc[self._last_idx]
        year = int(row['year']) + 2000
        month = int(row['month'])
        day = int(row['day'])
        return date(year, month, day)

    def time(self) -> time:
        """Returns the play time"""
        try:
            hour = int(self._row['hour'])
            min = int(self._row['minute'])
            sec = int(self._row['second'])
            return time(hour, min, sec)
        except:
            return time(0, 0, 0)

    def start_recording(self) -> time:
        """Returns beginning of the recording"""
        try:
            row = self._df.iloc[0]
            hour = int(row['hour'])
            min = int(row['minute'])
            sec = int(row['second'])
            return time(hour, min, sec)
        except:
            return time(0, 0, 0)

    def end_recording(self) -> time:
        """Returns end of the recording"""
        try:
            row = self._df.iloc[self._last_idx]
            hour = int(row['hour'])
            min = int(row['minute'])
            sec = int(row['second'])
            return time(hour, min, sec)
        except:
            return time(0, 0, 0)
        
    def __getitem__(self, item_name):
        """Returns the selected data. valid item names are defined in the dataformats file"""
        return self._row[item_name]

    def altitude_min(self):
        """Returns the lowest flight height"""
        return self._df["Pressure-altitude"].min()
        
    def altitude_max(self):
        """Returns the highest flight height"""
        return self._df["Pressure-altitude"].max()

    def altitude_series(self):
        """Returns the altitude series for plotting the barogram"""
        t = self._df.index / 100.0 / 60.0   # 100Hz ticks to minutes for the time axis
        altitude = self._df["Pressure-altitude"]
        return t, altitude

    def _check_idx_range(self):
        """Private: check, that index is in range"""
        if self._idx > self._last_idx:
            self._idx = self._last_idx
        elif self._idx < 0:
            self._idx = 0

