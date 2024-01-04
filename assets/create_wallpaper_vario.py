from PIL import Image, ImageDraw
import math

TO_RAD = math.pi / 180

class VarioWallpaper():
    def __init__(self, dims):
        self.__dict__.update(dims)
        self.img = Image.new(mode='1', size=(self.width, self.height), color=1)
        self.draw = ImageDraw.Draw(self.img)
        self.radius = self.diameter // 2
        self.center_x = self.radius + self.margin
        self.center_y = self.radius + self.margin

    def stroke(self, value):
        angle = value*self.angle*TO_RAD
        start_x = int(self.center_x - math.cos(angle)*self.radius)
        start_y = int(self.center_y - math.sin(angle)*self.radius)
        end_x = int(self.center_x - math.cos(angle)*(self.radius - self.stroke_len))
        end_y = int(self.center_y - math.sin(angle)*(self.radius - self.stroke_len))
        self.draw.line((start_x, start_y, end_x, end_y), width=self.stroke_width)

        # print precalculated coordinates, which can be copied into the rust code
        # this helps the embedded system to save time calculating sin() and cos()
        text_x = int(self.center_x - math.cos(angle)*(self.radius - self.stroke_text_pos)) + self.font_off_x
        text_y = int(self.center_y - math.sin(angle)*(self.radius - self.stroke_text_pos)) + self.font_off_y
        print(f'({text_x}, {text_y}, "{str(abs(value))}"),')

    def generate(self, path):
        print("Size", self.width, self.height)
        self.draw.arc((1, 1, self.diameter + 1, self.diameter + 1), 55, 305)
        for value  in (-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5):
            self.stroke(value)
        self.img.save(path)
        print(f"Save as '{path}'\n")
    
    def show(self):
        self.img.show()

DIMS_227_285 = {
    "width": 227,
    "height": 285,
    "diameter": 280,
    "margin": 2,
    "stroke_len": 18,
    "stroke_width": 4,
    "stroke_text_pos": 35,
    "font_off_x": -8,
    "font_off_y": 10,
    "angle": 25,
    "min_arc": 55,
    "max_arc": 305,
}

DIMS_240_320 = {
    "width": 240,
    "height": 320,
    "diameter": 314,
    "margin": 2,
    "stroke_len": 18,
    "stroke_width": 4,
    "stroke_text_pos": 35,
    "font_off_x": -8,
    "font_off_y": 10,
    "angle": 24,
    "min_arc": 60,
    "max_arc": 300,
}

wp = VarioWallpaper(DIMS_227_285)
wp.generate("assets/vario_wp_227x285.png")
wp = VarioWallpaper(DIMS_240_320)
wp.generate("assets/vario_wp_240x320.png")
