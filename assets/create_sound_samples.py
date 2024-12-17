import math

SAMPLES_COUNT = 40
MAX_AMPLITUDE = 4095
OFFSET = MAX_AMPLITUDE // 2

def silent_wave(_reduced):
    samples = []
    for _ in range(SAMPLES_COUNT):
        samples.append(str(OFFSET))
    return samples

def triangular_wave(reduced):
    value = OFFSET
    if reduced:
        delta = 4 * OFFSET / SAMPLES_COUNT / 10
    else:
        delta = 4 * OFFSET / SAMPLES_COUNT
    count = SAMPLES_COUNT // 4
    samples = []
    for _ in range(count):
        value += delta
        samples.append(str(int(value)))
    for _ in range(count):
        value -= delta
        samples.append(str(int(value)))
    for _ in range(count):
        value -= delta
        samples.append(str(int(value)))
    for _ in range(count):
        value += delta
        samples.append(str(int(value)))
    return samples

def sawtooth_wave(reduced):
    if reduced:
        delta = 2 * OFFSET / SAMPLES_COUNT / 10
    else:
        delta = 2 * OFFSET / SAMPLES_COUNT
    count = SAMPLES_COUNT // 2
    samples = []

    value = OFFSET
    for _ in range(count):
        value += delta
        samples.append(str(int(value)))

    if reduced:
        value = OFFSET - OFFSET / 10
    else:
        value = OFFSET

    for _ in range(count):
        value += delta
        samples.append(str(int(value)))
    return samples

def rectangular_wave(reduced):
    if reduced:
        delta = OFFSET / 10
    else:
        delta = OFFSET
    count = SAMPLES_COUNT // 2
    samples = []
    for _ in range(count):
        samples.append(str(int(OFFSET + delta)))
    for _ in range(count):
        samples.append(str(int(OFFSET - delta)))
    return samples
    
def sine_wave(reduced):
    if reduced:
        factor = 204.7
    else:
        factor = 2047
    count = SAMPLES_COUNT
    dy = 2 * math.pi / count
    samples = []
    y = 0
    for _ in range(count):
        y += dy
        samples.append(str(int(OFFSET + factor * math.sin(y))))
    return samples

def print_samples(f, reduced, name):
    samples = f(reduced)
    print("#[allow(unused)]")
    print(f"const {name}: [u16; SAMPLES_CNT] = [" )
    print("    " + ", ".join(samples))
    print("];\n")

print_samples(silent_wave, False, "SILENT_WAVE")
print_samples(triangular_wave, False, "TRIANGULAR_WAVE")
print_samples(triangular_wave, True, "TRIANGULAR_20DB_WAVE")
print_samples(sawtooth_wave, False, "SAWTOOTH_WAVE")
print_samples(sawtooth_wave, True, "SAWTOOTH_20DB_WAVE")
print_samples(rectangular_wave, False, "RECTANGULAR_WAVE")
print_samples(rectangular_wave, True, "RECTANGULAR_20DB_WAVE")
print_samples(sine_wave, False, "SINE_WAVE")
print_samples(sine_wave, True, "SINE_20DB_WAVE")