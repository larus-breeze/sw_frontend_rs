from urllib.request import urlopen

class Glider():
    def load_from_line(self, cpp_line, comment):
        line = cpp_line.replace(b'  { _T(', b'').replace(b'),', b',').replace(b' },', b'').replace(b'"', b'')
        line = line[:line.find(b' //')]
        vars = line.split(b', ')
        self.load_from_list(vars, comment)
        return self

    def load_from_list(self, vars, comment):
        self.name = vars[0].decode("utf-8")
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
    [b"LS-3 WL", 396, 121, 80, -0.604, 105, -0.700, 180, -1.939, 10.5, 250/3.6, 108, 295],
    [b"ASG-32", 807, 125, 100, -0.582, 126, -0.648, 185, -1.450, 15.7, 250/3.6, 120, 650],
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
            print(f"Could nor imprt Line {line}")
            pass

for line in ADDITONAL:
    glider = Glider().load_from_list(line, "self added")
    gliders.append(glider)


template = f"""// created by create_polar_store.py

use super::BasicGliderData;

pub const CONST_POLAR_COUNT: usize = {len(gliders)};

#[allow(unused)]
pub const POLARS: [BasicGliderData; CONST_POLAR_COUNT] = [
"""

with open("core/src/flight_physics/polar_store.rs", "w") as f:
    f.write(template)
    for idx, glider in enumerate(sorted(gliders)):
        f.write(glider.to_string(idx))
    f.write("];")