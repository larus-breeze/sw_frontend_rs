from PIL import Image, ImageDraw, ImageFont
import math

TO_RAD = math.pi / 180

class VarioWallpaper():
    def __init__(self, dims):
        self.__dict__.update(dims)
        self.img = Image.new(mode='1', size=(self.width, self.height), color=1)
        self.draw = ImageDraw.Draw(self.img)
        self.radius = self.diameter / 2 + 1
        self.center_x = self.radius
        self.center_y = self.radius
        self.font = ImageFont.truetype("assets/Arial_Bold.ttf", self.font_size)

    def stroke(self, value):
        # draw stroke
        angle = value*self.angle*TO_RAD
        start_x = int(self.center_x - math.cos(angle)*(self.radius+1))
        start_y = int(self.center_y - math.sin(angle)*(self.radius+1))
        end_x = int(self.center_x - math.cos(angle)*(self.radius - self.stroke_len))
        end_y = int(self.center_y - math.sin(angle)*(self.radius - self.stroke_len))
        self.draw.line((start_x, start_y, end_x, end_y), width=self.stroke_width, fill=1)

        # draw number
        text_x = int(self.center_x - math.cos(angle)*(self.radius - self.stroke_text_pos)) + self.font_off_x
        text_y = int(self.center_y - math.sin(angle)*(self.radius - self.stroke_text_pos)) + self.font_off_y
        self.draw.text((text_x, text_y), str(abs(value)), font=self.font, fill=1)

    def generate(self, path):
        self.draw.arc((0, 0, self.diameter, self.diameter), self.min_arc, self.max_arc, width=self.margin, fill=0)
        self.draw.arc((self.dx, self.dy, self.margin+self.dx, self.margin+self.dy), 0, 360, width=self.margin, fill=0)
        self.draw.arc((self.dx, self.height - self.margin - self.dy - 1, self.margin+self.dx, self.height - self.dy - 1), 0, 360, width=self.margin, fill=0)
        for value  in (-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5):
            self.stroke(value)
        self.img.save(path)
        print(f"Save as '{path}'")
    
    def show(self):
        self.img.show()

DIMS_227_285 = {
    "width": 227,
    "height": 285,
    "diameter": 284,
    "margin": 39,
    "dx": -100,
    "dy": -100,
    "font_size": 25,
    "stroke_len": 12,
    "stroke_width": 6,
    "stroke_text_pos": 27,
    "font_off_x": -6,
    "font_off_y": -14,
    "angle": 25,
    "min_arc": 0,
    "max_arc": 360,
}

DIMS_240_320 = {
    "width": 240,
    "height": 320,
    "diameter": 319,
    "margin": 45,
    "dx": -100,
    "dy": -100,
    "font_size": 26,
    "stroke_len": 15,
    "stroke_width": 7,
    "stroke_text_pos": 31,
    "font_off_x": -5,
    "font_off_y": -14,
    "angle": 24,
    "min_arc": 0,
    "max_arc": 360,
}

DIMS_480_480 = {
    "width": 480,
    "height": 480,
    "diameter": 479,
    "margin": 70,
    "dx": 325,
    "dy": 38,
    "font_size": 40,
    "stroke_len": 22,
    "stroke_width": 10,
    "stroke_text_pos": 48,
    "font_off_x": -10,
    "font_off_y": -22,
    "angle": 25,
    "min_arc": 55,
    "max_arc": 305,
}

wp = VarioWallpaper(DIMS_227_285)
wp.generate("assets/size_227x285/wp_vario.png")
wp = VarioWallpaper(DIMS_240_320)
wp.generate("assets/size_240x320/wp_vario.png")
wp = VarioWallpaper(DIMS_480_480)
wp.generate("assets/size_480x480/wp_vario.png")
