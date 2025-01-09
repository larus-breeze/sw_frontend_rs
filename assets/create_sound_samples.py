import math

SAMPLES_COUNT = 20
MAX_AMPLITUDE = 4095
HALF_AMPLITUDE = MAX_AMPLITUDE // 2

def triangular_wave():
    value = 0.0
    delta = 4 * HALF_AMPLITUDE / SAMPLES_COUNT
    count = SAMPLES_COUNT // 4
    samples = []
    for _ in range(count):
        value += delta
        samples.append(value)
    for _ in range(count):
        value -= delta
        samples.append(value)
    for _ in range(count):
        value -= delta
        samples.append(value)
    for _ in range(count):
        value += delta
        samples.append(value)
    return samples

def sawtooth_wave():
    delta = 2 * HALF_AMPLITUDE / SAMPLES_COUNT
    count = SAMPLES_COUNT // 2
    samples = []

    value = 0.0
    for _ in range(count):
        value += delta
        samples.append(value)

    value = float(-HALF_AMPLITUDE - delta)

    for _ in range(count):
        value += delta
        samples.append(value)
    return samples

def rectangular_wave():
    delta = float(HALF_AMPLITUDE)
    count = SAMPLES_COUNT // 2
    samples = []
    for _ in range(count):
        samples.append( + delta)
    for _ in range(count):
        samples.append( - delta)
    return samples
    
def sine_wave():
    factor = 2047.0
    count = SAMPLES_COUNT
    dy = 2 * math.pi / count
    samples = []
    y = 0.0
    for _ in range(count):
        y += dy
        samples.append(factor * math.sin(y))
    return samples

def volume():
    samples = []
    for idx in range(SAMPLES_COUNT):
        samples.append(0.9659363**(19 - idx))
    return samples

def get_samples(f, name, exact):
    to_store = []
    
    if exact:
        for sample in f():
            to_store.append(f"{sample:0.5f}")
    else:
        for sample in f():
            to_store.append(f"{sample:0.1f}")
    
    s = '\n'
    s += "#[allow(unused)]\n"
    s += f"pub const {name}: [f32; SAMPLES_COUNT] = [\n" 
    s += "    " + ", ".join(to_store)
    s += "];\n"
    return s

def get_file_content():
    content = "// Created by 'create_sound_samples.py' - do not edit\n\n"
    content += f"pub const SAMPLES_COUNT: usize = {SAMPLES_COUNT};\n"
    content += get_samples(triangular_wave, "TRIANGULAR_WAVE", False)
    content += get_samples(sawtooth_wave, "SAWTOOTH_WAVE", False)
    content += get_samples(rectangular_wave, "RECTANGULAR_WAVE", False)
    content += get_samples(sine_wave, "SINE_WAVE", False)
    content += get_samples(volume, "VOLUME_FACTORS", True)
    return content

def write_to_file(file_name, content):
    with open(file_name, 'w') as f:
        f.write(content)
    print(f"file '{file_name}' created")

content = get_file_content()
write_to_file('device/larus_frontend_v1/src/utils/samples.rs', content)
write_to_file('device/larus_frontend_v2/src/utils/samples.rs', content)
