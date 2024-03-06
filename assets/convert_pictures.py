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
import struct, os

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
        height, width = img.size
        print(f"Reading '{in_file}': format {img.format}, size {img.size}, mode {img.mode}")

        if version==1:
            with open(out_path, "wb") as f:
                src_px = img.load()
                width, height = img.size

                f.write(struct.pack("<HHHH", 1, self.width, height, len(color_dict)))
                size = 8

                for src_color in color_dict.keys():
                    dst_color = color_dict[src_color]
                    dest_px = []
                    for y in range(height):
                        for x in range(width):
                            if src_px[x, y] == src_color:
                                idx = x + ofs_x + (y+ofs_y)*self.width
                                dest_px.append(idx)
                    size += 4
                    f.write(struct.pack("<HH", dst_color, len(dest_px)))
                    size += len(dest_px)*2
                    for idx in dest_px:
                        f.write(struct.pack("<H", idx))

            print(f"File '{out_path}' {size} bytes written")
        elif version==2:
            with open(out_path, "wb") as f:
                src_px = img.load()
                width, height = img.size

                f.write(struct.pack("<LLLL", 2, self.width, height, len(color_dict)))
                size = 16

                for src_color in color_dict.keys():
                    dst_color = color_dict[src_color]
                    dest_px = []
                    for y in range(height):
                        for x in range(width):
                            if src_px[x, y] == src_color:
                                idx = x + ofs_x + (y+ofs_y)*self.width
                                dest_px.append(idx)
                    size += 8
                    f.write(struct.pack("<LL", dst_color, len(dest_px)))
                    size += len(dest_px)*4
                    for idx in dest_px:
                        f.write(struct.pack("<L", idx))

            print(f"File '{out_path}' {size} bytes written")
        else:
            raise ValueError("Format version unknown")


DARK_GRAY = 24
WHITE = 135

lif_type1 = LifGen(227, 285, 'assets/size_227x285', 'core/assets/size_227x285')
lif_type1.generate(1, 'glider.png', {0: DARK_GRAY}, 67, 117)
lif_type1.generate(1, 'north.png', {0: DARK_GRAY}, 71, 53)
lif_type1.generate(1, 'spiral.png', {0: DARK_GRAY})
lif_type1.generate(1, 'straight.png', {0: DARK_GRAY})
lif_type1.generate(1, 'vario_wallpaper.png', {0: DARK_GRAY})
lif_type1.generate(1, 'km_h.png', {0: DARK_GRAY})
lif_type1.generate(1, 'm_s.png', {0: DARK_GRAY})

lif_type1 = LifGen(240, 320, 'assets/size_240x320', 'core/assets/size_240x320')
lif_type1.generate(1, 'glider.png', {0: DARK_GRAY}, 82, 136)
lif_type1.generate(1, 'north.png', {0: DARK_GRAY}, 82, 58)
lif_type1.generate(1, 'spiral.png', {0: DARK_GRAY})
lif_type1.generate(1, 'straight.png', {0: DARK_GRAY})
lif_type1.generate(2, 'vario_wallpaper.png', {0: WHITE})
lif_type1.generate(1, 'km_h.png', {0: DARK_GRAY})
lif_type1.generate(1, 'm_s.png', {0: DARK_GRAY})
