clock = 100_000_000
lowest_fq = 200
highest_fq = 2000
samples = 20
amplitude = 2047
neutral = 2047

d1 = clock / samples / lowest_fq
d2 = clock / samples / highest_fq

print(d1, d2)

def calc_rel(fq):
    rr =  clock // samples // fq
    print(f"Frequency {fq}, Reload Register {rr}")
    return rr

calc_rel(200)
calc_rel(201)
calc_rel(440)
calc_rel(441)
calc_rel(2000)
calc_rel(2001)


print(f"const SAMPLES_CNT: usize = {samples};")

def print_vals(name, vals):
    print("#[allow(unused)]")
    print(f"const {name}_WAVE: [u16; SAMPLES_CNT] = {str(vals)};")

vals = []
for i in range(samples):
    vals.append(neutral)
print_vals("SILENT", vals)

vals = []
value = neutral
delta = 2 * amplitude / samples
for i in range(samples // 4):
    value += delta
    vals.append(int(value))
for i in range(samples // 2):
    value -= delta
    vals.append(int(value))
for i in range(samples // 4):
    value += delta
    vals.append(int(value))
print_vals("TRIANGULAR", vals)

vals = []
div = samples
value = neutral
delta = amplitude / samples
for i in range(samples // 2):
    value += delta
    vals.append(int(value))
value = neutral - amplitude // 2
for i in range(samples // 2):
    value += delta
    vals.append(int(value))
print_vals("SAWTOOTH", vals)

vals = []
for i in range(samples):
    if i < (samples // 2):
        vals.append(neutral + amplitude // 2)
    else:
        vals.append(neutral - amplitude // 2)
print_vals("RECTANGULAR", vals)

import math
def sin_val(x):
    return int(math.sin(x) * amplitude // 2 + neutral)

vals = []
delta = 2*math.pi/samples
x = delta
for i in range(samples):
    vals.append(sin_val(x))
    x += delta
print_vals("SINE", vals)
