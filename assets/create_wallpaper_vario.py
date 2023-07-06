from PIL import Image, ImageDraw
import math

DIAMETER = 280
MARGIN = 2
RADIUS = DIAMETER/2
CENTER_X = RADIUS + MARGIN
CENTER_Y = RADIUS + MARGIN
STROKE_LEN = 18
STROKE_WIDTH = 4
STROKE_TEXT_POS = 35
FONT_OFF_X = -8
FONT_OFF_Y = 10
TO_RAD = math.pi / 180

img = Image.new(mode='1', size=(227, 285), color=1)
draw = ImageDraw.Draw(img)

def vario_stroke(value):
    angle = value*25*TO_RAD
    start_x = int(CENTER_X - math.cos(angle)*RADIUS)
    start_y = int(CENTER_Y - math.sin(angle)*RADIUS)
    end_x = int(CENTER_X - math.cos(angle)*(RADIUS - STROKE_LEN))
    end_y = int(CENTER_Y - math.sin(angle)*(RADIUS - STROKE_LEN))
    draw.line((start_x, start_y, end_x, end_y), width=STROKE_WIDTH)

    # print precalculated coordinates, which can be copied into the rust code
    # this helps the embedded system to save time calculating sin() and cos()
    text_x = int(CENTER_X - math.cos(angle)*(RADIUS - STROKE_TEXT_POS)) + FONT_OFF_X
    text_y = int(CENTER_Y - math.sin(angle)*(RADIUS - STROKE_TEXT_POS)) + FONT_OFF_Y
    print(f'({text_x}, {text_y}, "{str(abs(value))}"),')
    

draw.arc((1, 1, 281, 281), 55, 305)
for value  in (-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5):
    vario_stroke(value)

img.save("assets/vario_wallpaper.png")
img.show()