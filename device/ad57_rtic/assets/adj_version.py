#!/bin/python3

import sys

class Version():
    def __init__(self, path_to_version_rs: str):
        self.path_to_version_rs = path_to_version_rs
        with open(path_to_version_rs, "r") as f:
            s = f.read()

        for line in s.splitlines():
            if "pub const SW_VERSION" in line:
                p1 = line.find('[') + 1
                p2 = line.rfind(']')
                version_str = line[p1:p2]

        splitted = version_str.split(', ')
        self.major = int(splitted[0])
        self.minor = int(splitted[1])
        self.patch = int(splitted[2])
        self.buildindex = int(splitted[3])

    def inc_major(self):
        self.major += 1
        self.minor = 0
        self.patch = 0
        self.buildindex = 0

    def inc_minor(self):
        self.minor += 1
        self.patch = 0
        self.buildindex = 0

    def inc_patch(self):
        self.patch += 1
        self.buildindex = 0

    def inc_buildindex(self):
        self.buildindex += 1

    def to_str(self, with_buildindex=True):
        if with_buildindex:
            return "%d.%d.%d.%d" % (self.major, self.minor, self.patch, self.buildindex)
        else:
            return "%d.%d.%d" % (self.major, self.minor, self.patch)
        
    def to_arr_cont(self):
            return "%d, %d, %d, %d" % (self.major, self.minor, self.patch, self.buildindex)

    def adj_pack_toml(self, path):
        with open(path, "r") as f:
            s = f.read()
        
        with open(path, "w") as fw:
            for line in s.splitlines():
                if "sw_version" in line:
                    idx = line.find('"') + 1
                    line = line[:idx] + self.to_str() + '"'
                fw.write(line+"\n")

    def adj_cargo_toml(self, path):
        with open(path, "r") as f:
            s = f.read()
        
        with open(path, "w") as fw:
            for line in s.splitlines():
                if "version = " in line[:10]:
                    idx = line.find('"') + 1
                    line = line[:idx] + self.to_str(with_buildindex=False) + '"'
                fw.write(line+"\n")

    def adj_version_rs(self):
        with open(self.path_to_version_rs, "r") as f:
            s = f.read()
        
        with open(self.path_to_version_rs, "w") as fw:
            for line in s.splitlines():
                if "pub const SW_VERSION" in line:
                    idx = line.find('[') + 1
                    line = line[:idx] + self.to_arr_cont() + ']};'
                fw.write(line+"\n")

print("Adjust Version")

version = Version("../../core/src/utils/version.rs")
old_version = version.to_str()

if len(sys.argv) == 1:
    version.inc_buildindex()
elif len(sys.argv) == 2:
    if sys.argv[1] == "major":
        version.inc_major()
    elif sys.argv[1] == "minor":
        version.inc_minor()
    elif sys.argv[1] == "patch":
        version.inc_patch()
    else:
        print("Error, wrong argument (not major, minor or patch)")
        sys.exit(1)
else:
    print("Error: Wrong arguments")
    sys.exit(1)

version.adj_cargo_toml("Cargo.toml")
version.adj_pack_toml("pack.toml")
version.adj_version_rs()

new_version = version.to_str()
print(f"Version changed {old_version} --> {new_version}")
