# Description of the image data format "lif"
#
# u16 type = 1: absolute index on framebuffer
# u16 width: width of the display
# u16 height: height of the display
# u16 colors: number of colors
# u16 color_1: First color
# u16 px_count: Number of pixels of this color
# u16 px_1: first pixel
# u16 px_2: second pixel
# u16 ...: more pixel
# u16 color_2:
# ...


from PIL import Image
import struct

DISPLAY_WIDTH = 227
DISPLAY_HEIGHT = 285

def gen_type_1(in_file, out_file, color_dict, ofs_x=0, ofs_y=0):
    img = Image.open(in_file)
    height, width = img.size
    print(f"Reading '{in_file}': format {img.format}, size {img.size}, mode {img.mode}")

    with open(out_file, "wb") as f:
        src_px = img.load()
        width, height = img.size

        f.write(struct.pack("<HHHH", 1, DISPLAY_WIDTH, height, len(color_dict)))
        size = 8

        for src_color in color_dict.keys():
            dst_color = color_dict[src_color]
            dest_px = []
            for y in range(height):
                for x in range(width):
                    if src_px[x, y] == src_color:
                        idx = x + ofs_x + (y+ofs_y)*DISPLAY_WIDTH
                        dest_px.append(idx)
            size += 4
            f.write(struct.pack("<HH", dst_color, len(dest_px)))
            size += len(dest_px)*2
            for idx in dest_px:
                f.write(struct.pack("<H", idx))

    print(f"File '{out_file}' {size} bytes written")


DARK_GRAY = 24
YELLOW = 137

gen_type_1("assets/size_227x285/glider.png", "core/assets/glider227.lif", {0: DARK_GRAY}, 67, 117)
gen_type_1("assets/size_227x285/north.png", "core/assets/compass227.lif", {0: DARK_GRAY}, 71, 53)
gen_type_1("assets/size_227x285/spiral.png", "core/assets/spiral227.lif", {0: DARK_GRAY})
gen_type_1("assets/size_227x285/straight.png", "core/assets/straight227.lif", {0: DARK_GRAY})
gen_type_1("assets/size_227x285/vario_wallpaper.png", "core/assets/vario_wallpaper227.lif", {0: DARK_GRAY})
gen_type_1("assets/size_227x285/km_h.png", "core/assets/km_h.lif", {0: DARK_GRAY})
gen_type_1("assets/size_227x285/m_s.png", "core/assets/m_s.lif", {0: DARK_GRAY})
