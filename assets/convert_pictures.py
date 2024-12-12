# Description of the image data format "lif"
#
# All data is stored least significant byte first
#
# Type 1
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
#
# Type 2
#
# u32 type = 2: absolute index on framebuffer
# u32 width: width of the display
# u32 height: height of the display
# u32 colors: number of colors
# u32 color_1: First color
# u32 px_count: Number of pixels of this color
# u32 px_1: first pixel
# u32 px_2: second pixel
# u32 ...: more pixel
# u32 color_2:
# ...
#
# Type 3
#
# u16 type = 3: absolute index on framebuffer
# u16 width: width of the display
# u16 height: height of the display
# u8 colors: number of colors
# u8 color[1]
# u8 color[2]
# u8 ..
# [u8]  picture data
#
# 0b00xx_xxxx plot x pixells with foreground color
# 0b01xx_xxxx skip x pixels
# 0b10xx_xxxx skip x*64 pixels
# 0b11xx_xxxx select foreground color x


from PIL import Image
import struct, os

DARK_GRAY = 24
GOLD = 47
LIME = 75
RED = 112
WHITE = 135

BACKGROUND = 255

class LifGen():
    def __init__(self, width, height, src_dir, dst_dir):
        self.width = width
        self.height = height
        self.src_dir = src_dir
        self.dst_dir = dst_dir

    def generate(self, version, in_file, color_dict, ofs_x=0, ofs_y=0):
        in_path = os.path.join(self.src_dir, in_file)
        out_path = os.path.join(self.dst_dir, in_file[:-3] + 'lif')
        img = Image.open(in_path)
        width, height = img.size
        print(f"Reading '{in_file}': format {img.format}, size {img.size}, mode {img.mode}")

        src_px = img.load()

        # check if all colors are part of color_dict
        src_colors = []
        for y in range(height):
            for x in range(width):
                color = src_px[x, y]
                if color not in src_colors:
                    src_colors.append(color)
        for color in src_colors:
            if color not in color_dict:
                print(f"*** Error color '{color} is not included in color_dict")
                exit(1)

        if version==1:
            with open(out_path, "wb") as f:
                f.write(struct.pack("<HHHH", 1, self.width, height, len(color_dict)-1))
                file_size = 8

                for src_color in color_dict.keys():
                    dst_color = color_dict[src_color]
                    if dst_color != BACKGROUND:
                        dest_px = []
                        for y in range(height):
                            for x in range(width):
                                if src_px[x, y] == src_color:
                                    idx = x + ofs_x + (y+ofs_y)*self.width
                                    dest_px.append(idx)
                        file_size += 4
                        f.write(struct.pack("<HH", dst_color, len(dest_px)))
                        file_size += len(dest_px)*2
                        for idx in dest_px:
                            f.write(struct.pack("<H", idx))

            print(f"File '{out_path}' {file_size} bytes written")

        elif version==2:
            with open(out_path, "wb") as f:
                f.write(struct.pack("<LLLL", 2, self.width, height, len(color_dict)-1))
                file_size = 16

                for src_color in color_dict.keys():
                    dst_color = color_dict[src_color]
                    if dst_color != BACKGROUND:
                        dest_px = []
                        for y in range(height):
                            for x in range(width):
                                if src_px[x, y] == src_color:
                                    idx = x + ofs_x + (y+ofs_y)*self.width
                                    dest_px.append(idx)
                        file_size += 8
                        f.write(struct.pack("<LL", dst_color, len(dest_px)))
                        file_size += len(dest_px)*4
                        for idx in dest_px:
                            f.write(struct.pack("<L", idx))

            print(f"File '{out_path}' {file_size} bytes written")

        elif version==3:

            with open(out_path, "wb") as f:
                last_color = None
                file_size = 0
                idx_backgroud = None

                def write_line(idx_col, px_cnt):
                    # print(f"write_line cnt {px_cnt}, col {idx_col}")
                    nonlocal last_color
                    nonlocal file_size
                    nonlocal idx_backgroud

                    if idx_col == idx_backgroud:
                        if px_cnt > 63:
                            x = px_cnt // 64
                            px_cnt -= x*64
                            f.write(struct.pack("B", 0b1000_0000 + x))
                            file_size += 1
                        if px_cnt > 0:
                            f.write(struct.pack("B", 0b0100_0000 + px_cnt))
                            file_size += 1
                    else:
                        if idx_col != last_color:
                            f.write(struct.pack("B", 0b1100_0000 | idx_col))
                            file_size += 1
                            last_color = idx_col

                        while px_cnt > 0:
                            if px_cnt > 63:
                                px_cnt -= 63
                                f.write(struct.pack("B", 63))
                            else:
                                f.write(struct.pack("B", px_cnt))
                                px_cnt = 0
                            file_size += 1

                f.write(struct.pack("<HHH", 3, self.width, height))
                file_size += 6

                f.write(struct.pack("B",  len(color_dict)))
                for c_nr in sorted(color_dict):
                    color = color_dict[c_nr]
                    f.write(struct.pack("B", color))
                    if color == BACKGROUND:
                        idx_backgroud = c_nr
                file_size += len(color_dict) + 1

                idx_col = None
                px_cnt = 0
                delta = self.width - width

                for y in range(height):
                    for x in range(width):
                        if idx_col == None:
                            idx_col = src_px[x, y]
                            px_cnt = 1
                        else:
                            if idx_col == src_px[x, y]:
                                px_cnt += 1
                            else:
                                write_line(idx_col, px_cnt)
                                idx_col = src_px[x, y]
                                px_cnt = 1

                    if idx_col == idx_backgroud:
                        write_line(idx_col, px_cnt + delta)
                    else:
                        write_line(idx_col, px_cnt)
                        write_line(idx_backgroud, delta)
                    idx_col = None

                print(f"File '{out_path}' {file_size} bytes written")

        else:
            raise ValueError("Format version unknown")

lif_gen = LifGen(227, 285, 'assets/size_227x285', 'device/air_avionics_ad57/assets')
lif_gen.generate(3, 'bat_empty.png', {0: RED, 1: BACKGROUND})
lif_gen.generate(3, 'bat_full.png', {0: DARK_GRAY, 1: LIME, 2: BACKGROUND})
lif_gen.generate(3, 'bat_half.png', {0: DARK_GRAY, 1: GOLD, 2: BACKGROUND})
lif_gen.generate(3, 'glider.png', {0: DARK_GRAY, 1: BACKGROUND}),
lif_gen.generate(3, 'north.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'spiral.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'straight.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'km_h.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'm_s.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'sat.png', {0: DARK_GRAY, 2: BACKGROUND})
lif_gen.generate(3, 'wp_vario.png', {0: DARK_GRAY, 255: BACKGROUND})
lif_gen.generate(3, 'wp_horizon.png', {0: DARK_GRAY, 255: BACKGROUND})

lif_gen = LifGen(240, 320, 'assets/size_240x320', 'device/larus_frontend_v1/assets')
lif_gen.generate(3, 'bat_empty.png', {0: DARK_GRAY, 1: LIME, 2: BACKGROUND})
lif_gen.generate(3, 'bat_full.png', {0: DARK_GRAY, 1: LIME, 2: BACKGROUND})
lif_gen.generate(3, 'bat_half.png', {0: DARK_GRAY, 1: GOLD, 2: BACKGROUND})
lif_gen.generate(3, 'glider.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'north.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'spiral.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'straight.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'km_h.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'm_s.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'sat.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'wp_vario.png', {0: WHITE, 255: BACKGROUND})
lif_gen.generate(3, 'wp_horizon.png', {0: WHITE, 255: BACKGROUND})

lif_gen = LifGen(480, 480, 'assets/size_480x480', 'device/larus_frontend_v2/assets')
lif_gen.generate(3, 'bat_empty.png', {0: DARK_GRAY, 1: RED, 2: BACKGROUND})
lif_gen.generate(3, 'bat_full.png', {0: DARK_GRAY, 1: LIME, 2: BACKGROUND})
lif_gen.generate(3, 'bat_half.png', {0: DARK_GRAY, 1: GOLD, 2: BACKGROUND})
lif_gen.generate(3, 'glider.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'north.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'spiral.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'straight.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'km_h.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'm_s.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'sat.png', {0: DARK_GRAY, 1: BACKGROUND})
lif_gen.generate(3, 'wp_vario.png', {0: WHITE, 255: BACKGROUND})
lif_gen.generate(3, 'wp_horizon.png', {0: WHITE, 255: BACKGROUND})
