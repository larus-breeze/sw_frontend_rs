import os

PICS = [
    "normal-0.png",
    "normal-1.png",
    "normal-2.png",
    "normal-3.png",
    "club-0.png",
    "club-1.png",
    "club-2.png",
    "club-3.png",

    "spiral.png",
    "straight.png",

    "tas.png",
    "speed_to_fly.png",
    "flight_level.png",
    "drift_angle.png",
    "avg_climb_rate.png",
    "true_course.png",
    "circle_delta.png",
    "circle_diameter.png",

    "km_h.png",
    "mph.png",
    "kt.png",
    "m_s.png",
    "fpm.png",
    "fpm-100.png",
    "m.png",
    "ft.png",
    "V.png",
    "battery.png",

    "g-load.png",
    "pitch.png",
    "roll_left.png",
    "roll_right.png",
    "slip.png",
    "yaw.png",
    "g.png",
    "Ve.png",
    "ias.png",

    "normal-0.png",
    "normal-1.png",
    "normal-2.png",
    "normal-3.png",
    "club-0.png",
    "club-1.png",
    "club-2.png",
    "club-3.png",
]

EXPORT_DIRS = [
    ("assets/size_227x285", "85"),
    ("assets/size_240x320", "85"),
    ("assets/size_480x480", "125"),
]

def export_png(source, f_name, dir_name, dpi):
    cmd = f"inkscape --export-id={f_name} --export-filename={dir_name}/{f_name} --export-png-color-mode=Gray_1 --export-dpi={dpi} {source}"
    print(f"  writing {dir_name}/{f_name}")
    os.system(cmd)

print("create_items.py")
for f_name in PICS:
    for dir_name, dpi in EXPORT_DIRS:
        export_png("assets/items.svg", f_name, dir_name, dpi)
