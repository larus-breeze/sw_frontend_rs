from PIL import Image, ImageDraw, ImageFont
import math

TO_RAD = math.pi / 180

class EditorWallpaper():
    def __init__(self, dims):
        self.__dict__.update(dims)
        self.img = Image.new(mode='P', size=(self.width, self.height), color=0)
        self.img.putpalette(b"\xff\xff\xff\x80\x80\x80\x00\x00\x00")
        self.draw = ImageDraw.Draw(self.img)
        print(self.img.getpalette())
        self.radius = self.diameter / 2 + 1
        self.center_x = self.radius - 1
        self.center_y = self.radius - 1

    def generate(self, path):
        self.draw.circle((self.center_x, self.center_y), self.radius - self.margin - 1, outline=1, fill=2, width=3)
        self.img.save(path)
        print(f"Save as '{path}'")
    
    def show(self):
        self.img.show()

DIMS_227_285 = {
    "width": 227,
    "height": 285,
    "diameter": 284,
    "margin": 39,
}

DIMS_240_320 = {
    "width": 240,
    "height": 320,
    "diameter": 319,
    "margin": 45,
}

DIMS_480_480 = {
    "width": 480,
    "height": 480,
    "diameter": 479,
    "margin": 70,
}

wp = EditorWallpaper(DIMS_227_285)
wp.generate("assets/size_227x285/wp_editor.png")
wp = EditorWallpaper(DIMS_240_320)
wp.generate("assets/size_240x320/wp_editor.png")
wp = EditorWallpaper(DIMS_480_480)
wp.generate("assets/size_480x480/wp_editor.png")
