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


len = 1.0

l_2 = len / 2
ah = len / 6
w = len / 20
px = [
    (0, -l_2),
    (ah, ah - l_2),
    (w, ah - l_2),
    (w, l_2),
    (-w, l_2),
    (-w, ah - l_2),
    (-ah, ah - l_2),
]

print("const ARROW_VALS: [[f32; 2]; 7] = [")
for x, y in px:
    l, a = to_l_rad(x, y)
    print(f"    [{l}, {a}],")
print("];\n")

"""for idx, p in enumerate(px):
    x, y = p
    print(f"p{idx+1} = ({x:5.1f}, {y:5.1f})")
    l, a = to_l_rad(x, y)
    print(f"     ({l:5.1f}, {a:6.3f} -> {a*180/math.pi:3.0f}°)")
    x1, y1 = to_x_y(l, a)
    print(f"     ({x1:5.1f}, {y1:5.1f})")
    x2, y2 = to_x_y(l, a + math.pi / 2)
    print(f"     ({x2:5.1f}, {y2:5.1f})")
    x3, y3 = to_x_y(l, a + 2.5*math.pi)
    print(f"     ({x3:5.1f}, {y3:5.1f})")

"""

