

class Item():
    def __init__(self, raw_idx, name):
        self.raw_idx = raw_idx
        self.name = name

    def __lt__(self, other):
        return self.name < other.name

    def __repr__(self):
        return f"<Item({self.raw_idx}, {self.name})>"

def parse_polar_store(file_name):
    items = []
    idx = 0
    with open(file_name, 'r') as f:
        for line in f.readlines():
            val = line[8:line.find(',')].split(': ')
            if val[0] == 'name' and "&'static str" not in line:
                items.append(Item(idx, val[1].replace('"', '')))
                idx += 1 
    return items

template = """// Created by create_polar_idx.py
#![allow(clippy::all)]

pub const TO_SORTED: &'static[u8] = &[
@to_sorted@];

pub const TO_RAW: &'static[u8] = &[
@to_raw@];
"""

def create_polars_idx(file_name, items):
    to_sorted_dict = {}
    to_raw_dict = {}
    for sorted_idx, item in enumerate(sorted(items)):
        to_raw_dict[sorted_idx] = item.raw_idx
        to_sorted_dict[item.raw_idx] = sorted_idx

    to_sorted = ''
    to_raw = ''
    for idx in range(len(items)):
        to_sorted += f"    {to_sorted_dict[idx]}, // {items[idx].name}\n"
        to_raw += f"    {to_raw_dict[idx]}, //{items[to_raw_dict[idx]].name}\n"

    file_content = template \
        .replace('@to_sorted@', to_sorted) \
        .replace('@to_raw@', to_raw)
    
    with open(file_name, 'w') as f:
        f.write(file_content)




items = parse_polar_store('core/src/flight_physics/polar_store.rs')
create_polars_idx("core/src/flight_physics/polar_store_idx.rs", items)
