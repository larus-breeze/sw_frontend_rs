import math

def to_l_rad(x, y):
    l = math.sqrt(x*x + y*y)
    if y == 0:
        a = math.pi / 2 * (abs(x)/x)
    elif y < 0 and x >= 0:
        a = math.atan(-x/y)
    elif y > 0 and x >= 0:
        a = math.pi - math.atan(x/y)
    elif y > 0 and x < 0:
        a = math.atan(-x/y) - math.pi
    elif y < 0 and x < 0:
        a = math.atan(-x/y)
    return (l, a)

def to_x_y(l, a):
    x = math.sin(a) * l 
    y = -math.cos(a) * l
    return (x, y)


def print_coord_array(name, coords):
    print("// Normalized polar coordinates created by 'create_sprite_coords.py'")
    c_cnt = len(coords)
    print(f"const {name}: [PolarCoordinate; {c_cnt}] = [")
    for x, y in coords:
        l, a = to_l_rad(x, y)
        print(f"    PolarCoordinate{{ len: {l}, alpha: {a} }},")
    print("];\n\n")


ARROW = (                   
    0.01,                   # scale
        [                   # x, y coords
            (0, -50),
            (16, -34),
            (5, -34),
            (5, 50),
            (-5, 50),
            (-5, -34),
            (-16, -34)
        ]
)


def arrow_xy_coords():
    l = 1.0

    l_2 = l / 2
    ah = l / 6
    w = l / 20
    return [
        (0, -l_2),
        (ah, ah - l_2),
        (w, ah - l_2),
        (w, l_2),
        (-w, l_2),
        (-w, ah - l_2),
        (-ah, ah - l_2),
    ]

print_coord_array("ARROW_PCOORDS", arrow_xy_coords())
