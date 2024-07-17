from PIL import Image, ImageDraw
import math

TO_RAD = math.pi / 180

class ArtificialHorizonWallpaper():
    def __init__(self, dims):
        self.__dict__.update(dims)
        self.img = Image.new(mode='1', size=(self.width, self.height), color=1)
        self.draw = ImageDraw.Draw(self.img)
        self.radius = self.width // 2
        self.center_x = self.radius
        self.center_y = self.radius
        self.ah_y_ofs = 0

    def comp_strokes(self):
        s_width = self.comp_stroke_width
        cx = self.radius
        dx = self.width // 10
        y = self.width

        self.draw.line((0, y, self.width, y), width=s_width)
        self.draw.line((cx, y, cx, y + self.comp_stroke_len), width=s_width)
        for m in [1, 2, 3, 4]:
            self.draw.line((cx+m*dx, y, cx+m*dx, y + self.comp_stroke_len), width=s_width)
            self.draw.line((cx-m*dx, y, cx-m*dx, y + self.comp_stroke_len), width=s_width)
        cnt = 0

    def ah_wallpaper(self):
        y_ofs = self.ah_y_ofs
        self.draw.arc((0, y_ofs, self.width, self.width + y_ofs), 210, 330, width=self.ah_stroke_width)

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
            )
            alpha += delta

        w = self.ah_point_width
        self.draw.arc((cx - w, cy - w, cx + w, cy + w), 0, 360, width=10)

        x1 = int(self.radius*0.2)
        x2 = int(self.radius*0.8)
        x3 = int(self.radius*0.05)
        x4 = x1 + self.ah_glider_width / 2
        y1 = int(self.radius*0.15)
        self.draw.line((cx - x1, cy, cx - x2, cy), width=self.ah_glider_width)
        self.draw.line((cx + x1, cy, cx + x2, cy), width=self.ah_glider_width)

        self.draw.line((cx - x2, cy, cx - x2 - x3, cy - y1), width=self.ah_glider_width)
        self.draw.line((cx + x2, cy, cx + x2 + x3, cy - y1), width=self.ah_glider_width)
        self.draw.arc((cx - x4, cy - x4, cx + x4, cy + x4), 0, 180, width=self.ah_glider_width)


    def generate(self, path):
        print("Size", self.width, self.height)
        self.ah_wallpaper()
        self.comp_strokes()
        self.img.save(path)
        print(f"Save as '{path}'\n")
    
    def show(self):
        self.img.show()

DIMS_227_285 = {
    "width": 227,
    "height": 285,
    "ah_stroke_len": 16,
    "ah_stroke_width": 2,
    "ah_point_width": 5,
    "ah_glider_width": 4,
    "comp_stroke_len": 15,
    "comp_stroke_width": 2,
}

DIMS_240_320 = {
    "width": 240,
    "height": 320,
    "ah_stroke_len": 18,
    "ah_stroke_width": 2,
    "ah_point_width": 5,
    "ah_glider_width": 4,
    "comp_stroke_len": 18,
    "comp_stroke_width": 2,
}

wp = ArtificialHorizonWallpaper(DIMS_227_285)
wp.generate("assets/arthorizon_wp_227x285.png")
wp = ArtificialHorizonWallpaper(DIMS_240_320)
wp.generate("assets/arthorizon_wp_240x320.png")
