#!../.venv/bin/python

import os

# 1. get version
import subprocess
import re
import os
import sys

log_file = open("make_manual.log", "w")

git_branch = subprocess.check_output("git rev-parse --abbrev-ref HEAD", shell=True).decode('utf-8')[:-1]
if not (len(sys.argv) > 1 and sys.argv[1]=='-i'): 
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

print("get version...")
with open("typst/version.typ", "w") as f:
    f.write(f'#let version = "Version v{first}.{second}.{third}.{build}"\n')

# 2. extract menus
print("create menus...")
subprocess.run(
    ["cargo", "run"], 
    cwd="../core/tools",
    text=True,
    check=True,
    stdout=log_file,
    stderr=subprocess.STDOUT,
)

# 3. make manuals
print("make manual-de...")
subprocess.run(
    ["typst", "compile", "manual-de.typ"], 
    cwd="typst", 
    text=True,
    check=True,
    stdout=log_file,
    stderr=subprocess.STDOUT,
)

print("make manual-en...")
subprocess.run(
    ["typst", "compile", "manual-en.typ"], 
    cwd="typst", 
    text=True,
    check=True,
    stdout=log_file,
    stderr=subprocess.STDOUT,
)

# 4. copy manuals and rename them
os.popen(f"rm -f *.pdf")

version_str = f"v{first}-{second}-{third}-{build}"
print("copying manual-de...")

os.popen(f"cp typst/manual-de.pdf manual_de_{version_str}.pdf")

print("copying manual-en...")
os.popen(f"cp typst/manual-en.pdf manual_en_{version_str}.pdf")

# finished
log_file.close()
