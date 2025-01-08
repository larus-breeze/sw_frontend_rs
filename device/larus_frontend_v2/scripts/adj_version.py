#!../../.venv/bin/python

import subprocess
import re
import os
import sys

git_branch = subprocess.check_output("git rev-parse --abbrev-ref HEAD", shell=True).decode('utf-8')[:-1]
if git_branch != 'master': 
    print(f"Branch is '{git_branch}' - not master, change branch first or use -i option!")
    sys.exit(1)

version_string = subprocess.check_output("git describe --always --dirty --tags", shell=True).decode('utf-8')
if not (len(sys.argv) > 1 and sys.argv[1]=='-i'): 
    if "dirty" in version_string:
        print("Project is not in a consistent state, please comit first or use -i option!")
        sys.exit(1)

first = 0xff
second = 0xff
third = 0xff
build = 0xff

try:
    # Try to match tag and build number
    match = re.match('v(?P<first>[0-9]*).(?P<second>[0-9]*).(?P<third>[0-9]*)-(?P<build>[0-9]*)-.*', version_string)
    first = int(match.group('first'))
    second = int(match.group('second'))
    third = int(match.group('third'))
    build = int(match.group('build'))

except Exception as e:
    # Try to match tag only from e newly created version.
    try:
        match = re.match('v(?P<first>[0-9]*).(?P<second>[0-9]*).(?P<third>[0-9]*).*', version_string)
        first = int(match.group('first'))
        second = int(match.group('second'))
        third = int(match.group('third'))
        build = 0

    except Exception as e:
        print("Something went wrong getting the TAG version information!: ", e)
        sys.exit(1)

git_hash = subprocess.check_output("git log -1 --format=%h", shell=True).decode('utf-8')[:-2]
git_time = subprocess.check_output("git show --no-patch --pretty=%cI", shell=True).decode('utf-8')[:-2]
git_tag = subprocess.check_output("git describe --tags", shell=True).decode('utf-8')[:-1]

print('sw_version: ', f'{first}-{second}-{third}-{build}')
print('git_hash:   ', git_hash)
print('git_time:   ', git_time)
print('git_tag:    ', git_tag)


print('create:      pack.toml')
with open("scripts/template_pack.toml", "r") as file:
    content = file.read()

content = content \
    .replace('@sw_version@', f'{first}.{second}.{third}.{build}') \
    .replace('@sw_version_fn@', f'{first}-{second}-{third}-{build}')

with open("pack.toml", "w") as file:
    file.write(content)


VERSION_FILE = 'src/utils/version.rs'
print("create      ", VERSION_FILE)
with open("scripts/template_version.rs", "r") as file:
    content = file.read()

content = content \
    .replace('[0xff, 0xff, 0xff, 0xff]', f'[{first}, {second}, {third}, {build}]') \
    .replace('@git_hash@', git_hash) \
    .replace('@git_time@', git_time) \
    .replace('@git_tag@', git_tag)

with open(VERSION_FILE, "w") as file:
    file.write(content)


print("Manipulate:  Cargo.toml")
with open("Cargo.toml", 'r') as file:
    content = file.read()

with open("Cargo.toml", "w") as file:
    for line in content.splitlines():
        if line[:10] == "version = ":
            line = f'version = "{first}.{second}.{third}"'
        file.write(line + '\n')