from urllib.request import urlopen

TRANSLATE_GLIDER_NAMES = {
    "401 Kestrel (17m)": "401 Kestrel 17m",
    "G 102 Club Astir IIIb": "G102 Club Astir",
    "G 102 Standard Astir III": "G102 Std Astir",
    "G 104 Speed Astir": "G104 Speed Astir",
    "H-201 Std Libelle": "H201 Std Libelle",
    "H-205 Club Libelle": "H205 Club Libelle",
    "Ka 4 Rhoenlerche": "Ka 4",
    "SZD-48-2 Jantar Std 2": "SZD-48-2 Jantar",
    "SZD-48-3 Jantar Std 3": "SZD-48-3 Jantar",
    "SZD-54-2 Perkoz (FT 17m)) /* flat tip */": "SZD-54-2 17m",
    "SZD-54-2 Perkoz (WL 17m)) /* winglet */": "SZD-54-2 17m WL",
    "SZD-54-2 Perkoz (WL 20m)) /* long winglet */": "SZD-54-2 20m WL",
    "SZD-9 bis 1E Bocian": "SZD-9-1E Bocian",
    "Ventus 2c (18m)": "Ventus 2c 18m",
    "Ventus 2cT (18m)": "Ventus 2cT 18m",
    "Ventus 2cx (18m)": "Ventus 2cx 18m",
    "Ventus 2cxT (18m)": "Ventus 2cxT 18m",
    "Ventus a/b (16.6m)": "Ventus a/b 16.6m",
}

class Glider():
    def load_from_line(self, cpp_line, comment):
        line = cpp_line.replace(b'  { _T(', b'').replace(b'),', b',').replace(b' },', b'').replace(b'"', b'')
        pos_slash = line.find(b' //')
        if pos_slash > 0:
            line = line[:pos_slash]
        vars = line.split(b', ')
        self.load_from_list(vars, comment)
        return self

    def load_from_list(self, vars, comment):
        self.name = vars[0].decode("utf-8")
        if self.name in TRANSLATE_GLIDER_NAMES:
            self.name = TRANSLATE_GLIDER_NAMES[self.name]
        self.reference_mass = float(vars[1])
        self.max_ballast = float(vars[2])
        self.v1 = float(vars[3])
        self.w1 = float(vars[4])
        self.v2 = float(vars[5])
        self.w2 = float(vars[6])
        self.v3 = float(vars[7])
        self.w3 = float(vars[8])
        self.wing_area = float(vars[9])
        # self.v_no = float(vars[10])*3.6 # v_no "normal operation, not max speed"
        self.max_speed = 250.0
        self.contest_handicap = int(vars[11])
        self.empty_mass = float(vars[12])
        self.comment = comment
        return self
    
    def __lt__(self, other):
        return self.name < other.name
    
    def to_string(self, no):
        r = "    BasicGliderData {\n"
        r += f"        // No {no},  {self.comment}\n"
        r += f'        name: "{self.name}",\n'
        r += f"        wing_area: {self.wing_area:.2f},\n"
        r += f"        max_speed: {self.max_speed:.1f},\n"
        r += f"        empty_mass: {self.empty_mass:.1f},\n"
        r += f"        max_ballast: {self.max_ballast:.1f},\n"
        r += f"        reference_weight: {self.reference_mass:.1f},\n"
        r += f"        handicap: {self.contest_handicap},\n"
        r += f"        polar_values: [[{self.v1:.1f}, {self.w1:.3f}], [{self.v2:.1f}, {self.w2:.3f}], [{self.v3:.1f}, {self.w3:.3f}]],\n"
        r += "    },\n"
        return r

ADDITONAL = (
    #  name ref_weight max_ballast w1 si1 w2 si2 w3 si3 wing_area max_speed handicap empty_mass
    [b"LS-3 WL", 396, 121, 80, -0.604, 105, -0.700, 180, -1.939, 10.5, 250/3.6, 108, 295],
    [b"ASG-32", 807, 125, 100, -0.582, 126, -0.648, 185, -1.450, 15.7, 250/3.6, 120, 650],
    [b"Ventus 2b 15m", 339, 200, 85, -0.576, 110, -0.648, 200, -2.230, 9.7, 250/3.6, 115, 248],
    [b"AS-33 18m", 400, 220, 97.2, -0.5109, 111.6, -0.5560, 180, -1.3694, 10.0, 270/3.6, 122, 285],
    [b"AS-33 15m", 352, 180, 86.4, -0.5834, 115.2, -0.6422, 180, -1.4728, 8.8, 270/3.6, 116, 275],
)



link = "https://raw.githubusercontent.com/XCSoar/XCSoar/master/src/Polar/PolarStore.cpp"
f = urlopen(link)
cpp_store = f.read()

gliders = []
for line in cpp_store.splitlines():
    if b'  { _T("' in line:
        try:
            glider = Glider().load_from_line(line, "imported from XCSoar")
            gliders.append(glider)
        except:
            print(f"Could nor import line {line}")
            pass

for line in ADDITONAL:
    glider = Glider().load_from_list(line, "self added")
    gliders.append(glider)


template = f"""// created by create_polar_store.py

use super::BasicGliderData;

pub const POLAR_COUNT: usize = @glider_count@;

#[allow(unused)]
pub const POLARS: [BasicGliderData; POLAR_COUNT] = [
@glider_data@];"""

glider_data = ""
for idx, glider in enumerate(sorted(gliders)):
    glider_data += glider.to_string(idx)


with open("core/src/flight_physics/polar_store.rs", "w") as f:
    polar_storre = template \
        .replace('@glider_count@', str(len(gliders))) \
        .replace('@glider_data@', glider_data)
    f.write(polar_storre)
