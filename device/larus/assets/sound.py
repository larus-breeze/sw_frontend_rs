clock = 100_000_000
lowest_fq = 200
highest_fq = 2000
oversampling = 20

d1 = clock / oversampling / lowest_fq
d2 = clock / oversampling / highest_fq

print(d1, d2)

def calc_rel(fq):
    rr =  clock // oversampling // fq
    print(f"Frequency {fq}, Reload Register {rr}")
    return rr

calc_rel(200)
calc_rel(201)
calc_rel(440)
calc_rel(441)
calc_rel(2000)
calc_rel(2001)


def print_vals(name, vals):
    print("#[allow(unused)]")
    print(f"const {name}_WAVE: [u16; SAMPLES_CNT] = {str(vals)};")

vals = []
value = 0.0
div = oversampling // 2
for i in range(div):
    value += 4095 / div
    vals.append(int(value))
for i in range(div):
    value -= 4095 / div
    vals.append(int(value))
print_vals("TRIANGULAR", vals)

vals = []
div = oversampling
value = 0
for i in range(oversampling):
    value += 4095 / div
    vals.append(int(value))
print_vals("SAWTOOTH", vals)

vals = []
for i in range(oversampling):
    if i < (oversampling // 2):
        vals.append(4095)
    else:
        vals.append(0)
print_vals("RECTANGULAR", vals)

import math
def sin_val(value):
    return int((math.sin(value)+1) * 2047)

vals = []
delta = 2*math.pi/oversampling
value = 0.0
for i in range(oversampling):
    vals.append(sin_val(value))
    value += delta
print_vals("SINE", vals)
