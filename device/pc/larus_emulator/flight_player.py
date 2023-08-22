from PyQt5 import QtCore, QtGui, QtWidgets
import sys, os

from emulator_ui import Ui_Dialog
from flight_data import FlightData

"""if __name__ == "__main__":
    import sys
    app = QtWidgets.QApplication(sys.argv)
    Dialog = QtWidgets.QDialog()
    ui = Ui_Dialog()
    ui.setupUi(Dialog)
    Dialog.show()
    sys.exit(app.exec_())"""

class Emulator(QtWidgets.QDialog):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.ui = Ui_Dialog()
        self.ui.setupUi(self)

        self.data = FlightData(5005)
        self.file_open = False
        self.tick_cnt = 0
        self.is_running = False
        self.blink = False

        self.setWindowTitle('Larus Flight Player')
        self.setWindowIcon(QtGui.QIcon('larus_breeze.png'))

        self.ui.pbStart.pressed.connect(self.startEmulator)
        self.ui.pbStop.pressed.connect(self.stopEmulator)
        self.ui.pbOpenFile.pressed.connect(self.openFile)
        self.ui.hsPlayerSpeed.valueChanged.connect(self.setPlayerSpeed)
        self.ui.hsEmulationTime.valueChanged.connect(self.setPlayerPos)

        self.led = QtGui.QPixmap("green_led.png")
        self.no_led = QtGui.QPixmap("no_led.png")
        self.ui.lbBlink.setPixmap(self.no_led)
        self.timer = QtCore.QTimer()
        self.timer.timeout.connect(self.tick_100ms)
        self.timer2 = QtCore.QTimer()
        self.timer2.timeout.connect(self.tick_1s)
        self.timer2.start(1000)
        self.setPlayerSpeed()
        self.baroWidget = self.ui.baroWidget

    def startEmulator(self):
        self.timer.start(100)
        self.is_running = True
        self.blink = True

    def stopEmulator(self):
        self.timer.stop()
        self.is_running = False
        self.ui.lbBlink.setPixmap(self.no_led)

    def tick_100ms(self):
        if self.file_open and self.is_running:
            self.data.tick()
            self.data.can_send_frames()

    def tick_1s(self):
        if self.file_open:
            self.data.nmea_send_frames()
            self.ui.lbFlightTimeA.setText(str(self.data.time()))
            if self.is_running:
                if self.blink:
                    self.ui.lbBlink.setPixmap(self.led)
                else:
                    self.ui.lbBlink.setPixmap(self.no_led)
                self.blink = not self.blink
                self.ui.hsEmulationTime.setValue(self.data.get_relative())
                self.ui.lbIasA.setText(f"{self.data['IAS']*3.6:3.0f}")
                self.ui.lbAltitudeA.setText(f"{self.data['Pressure-altitude']:4.0f}")

    def openFile(self):
        """Öffnet eine Datei und fügt in die SatusBar FrameFormat + Datei name hinzu"""
        fileName, x = QtWidgets.QFileDialog.getOpenFileName(
            self, "Open File", "", "Larus files (*.f110)")

        if fileName != '':
            self.stopEmulator()
            QtWidgets.QApplication.setOverrideCursor(QtCore.Qt.WaitCursor)
            self.data.from_file(fileName)
            self.ui.lbFileNameA.setText(fileName.split('/')[-1])
            self.ui.lbDateA.setText(str(self.data.date_of_flight()))
            self.ui.lbStartRecordingA.setText(str(self.data.start_recording()))
            self.ui.lbStopRecordingA.setText(str(self.data.end_recording()))
            self.ui.verticalLayout.removeWidget(self.baroWidget)
            self.baroWidget = self.data.getWidget(self)
            self.ui.verticalLayout.insertWidget(4, self.baroWidget)

            self.file_open = True
            self.is_running = False
            self.setPlayerPos()
            QtWidgets.QApplication.restoreOverrideCursor()

    def setPlayerSpeed(self):
        pos = self.ui.hsPlayerSpeed.value() # 1..20
        if pos < 10:
            speed = 0.1 * pos
        else:
            speed = (pos - 9) * 1.0
        self.ui.lbSpeed.setText(f"{speed:3.1f}")
        self.data.set_speed(speed)

    def setPlayerPos(self):
        if self.file_open:
            pos = self.ui.hsEmulationTime.value()   # 0..999
            self.data.set_relative(pos)
            self.ui.lbFlightTimeA.setText(str(self.data.time()))

if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    window = Emulator()
    window.resize(1150, 500)
    window.show()

    sys.exit(app.exec_())