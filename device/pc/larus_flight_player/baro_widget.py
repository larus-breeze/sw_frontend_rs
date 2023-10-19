import matplotlib

from PyQt5 import QtCore, QtWidgets

from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg
from matplotlib.figure import Figure

from flight_data import FlightData

matplotlib.use('Qt5Agg')

class BaroWidget(FigureCanvasQTAgg):

    def __init__(self, data: FlightData, parent=None, figsize=(1024, 200)):
        fig = Figure(figsize=figsize)
        self.axes = fig.add_subplot(111)
        super(BaroWidget, self).__init__(fig)
        
        self.axes.set_ylim(data.altitude_min(), data.altitude_max())
        t, altitude = data.altitude_series()
        self.axes.plot(t, altitude, label="Altitude")
