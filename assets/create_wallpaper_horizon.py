from PIL import Image, ImageDraw
import math

TO_RAD = math.pi / 180

class ArtificialHorizonWallpaper():
    def __init__(self, dims):
        self.__dict__.update(dims)
        #self.img = Image.new(mode='1', size=(self.width, self.height), color=1)
        self.img = Image.new(mode='P', size=(self.width, self.height), color=0)
        self.img.putpalette(b"\xff\xff\xff\x00\x00\x00\x80\x80\x80\x10\x10\x10")

        self.draw = ImageDraw.Draw(self.img)
        self.radius = self.width // 2
        self.center_x = self.radius
        self.center_y = self.radius
        self.ah_y_ofs = 0

    def level_tube(self):
        h = self.height / 5
        self.draw.rectangle((0, self.height - h, self.width, self.height), fill=3)

        my = int(self.height * 0.25)
        w = self.width // 2
        len = int(self.height * 0.65)
        radius = int(self.width * 0.035)
        alpha = 0.0
        delta = math.pi*0.1/180

        for idx in range(200):
            self.draw.circle((w + len*math.sin(alpha), my + len*math.cos(alpha)), radius, fill=2)
            self.draw.circle((w - len*math.sin(alpha), my + len*math.cos(alpha)), radius, fill=2)
            alpha += delta

        d = 3
        self.draw.line((w - radius - d, self.height - h, w - radius - d, self.height), width=self.ah_stroke_width, fill=3)
        self.draw.line((w + radius + d, self.height - h, w + radius + d, self.height), width=self.ah_stroke_width, fill=3)



    def ah_wallpaper(self):
        y_ofs = self.ah_y_ofs
        if self.show_arc_limited:
            self.draw.arc((0, y_ofs, self.width, self.width + y_ofs), 210, 330, width=self.ah_stroke_width, fill=1)
        else:
            self.draw.arc((0, y_ofs, self.width, self.width + y_ofs), 0, 360, width=self.ah_stroke_width, fill=1)

        r1 = self.radius - self.ah_stroke_len
        r2 = self.radius
        cx = self.center_x
        cy = self.ah_y_ofs + self.width // 2
        delta = math.pi*15/180
        alpha = -math.pi*240/180

        for idx in range(9):
            self.draw.line(
                (
                    cx + int(r1*math.sin(alpha)), 
                    cy + int(r1*math.cos(alpha)),
                    cx + int(r2*math.sin(alpha)), 
                    cy + int(r2*math.cos(alpha))
                ),
                width=self.ah_stroke_width,
                fill=1
            )
            alpha += delta

        w = self.ah_point_width
        self.draw.arc((cx - w, cy - w, cx + w, cy + w), 0, 360, width=10, fill=1)

        x1 = int(self.radius*0.2)
        x2 = int(self.radius*0.8)
        x3 = int(self.radius*0.05)
        x4 = x1 + self.ah_glider_width / 2
        y1 = int(self.radius*0.15)
        self.draw.line((cx - x1, cy, cx - x2 - 1, cy), width=self.ah_glider_width, fill=1)
        self.draw.line((cx + x1, cy, cx + x2 + 1, cy), width=self.ah_glider_width, fill=1)

        self.draw.line((cx - x2, cy + 1, cx - x2 - x3, cy - y1), width=self.ah_glider_width, fill=1)
        self.draw.line((cx + x2, cy + 1, cx + x2 + x3, cy - y1), width=self.ah_glider_width, fill=1)


    def generate(self, path):
        print("Size", self.width, self.height)
        self.ah_wallpaper()
        self.level_tube()
        self.img.save(path)
        print(f"Save as '{path}'\n")
    
    def show(self):
        self.img.show()

DIMS_227_285 = {
    "show_arc_limited": True,
    "width": 227,
    "height": 285,
    "bottom_line": 227,
    "ah_stroke_len": 16,
    "ah_stroke_width": 2,
    "ah_point_width": 5,
    "ah_glider_width": 4,
    "comp_stroke_len": 15,
    "comp_stroke_width": 2,
}

DIMS_240_320 = {
    "show_arc_limited": True,
    "width": 240,
    "height": 320,
    "bottom_line": 240,
    "ah_stroke_len": 18,
    "ah_stroke_width": 2,
    "ah_point_width": 5,
    "ah_glider_width": 4,
    "comp_stroke_len": 18,
    "comp_stroke_width": 2,
}

DIMS_480_480 = {
    "show_arc_limited": False,
    "width": 480,
    "height": 480,
    "bottom_line": 430,
    "ah_stroke_len": 27,
    "ah_stroke_width": 3,
    "ah_point_width": 7,
    "ah_glider_width": 6,
    "comp_stroke_len": -27,
    "comp_stroke_width": 3,
}

wp = ArtificialHorizonWallpaper(DIMS_227_285)
wp.generate("assets/arthorizon_wp_227x285.png")
wp = ArtificialHorizonWallpaper(DIMS_240_320)
wp.generate("assets/arthorizon_wp_240x320.png")
wp = ArtificialHorizonWallpaper(DIMS_480_480)
wp.generate("assets/arthorizon_wp_480x480.png")
