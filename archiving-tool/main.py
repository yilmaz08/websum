"""
This script is used to create an archive directory based on the archive.jsonc file.
It is not the websum tool, it is only used to update archives.

Steps to use:
- Edit the archive.jsonc file.
- Create a virtual environment and install the requirements.txt file.
- Run this script.

BE AWARE! You should run the script on /archiving-tool/ directory. Otherwise, it might cause loss of data.
"""

from jsoncomment import JsonComment
import shutil
import os

json_parser = JsonComment()

with open('archive.jsonc', 'r') as file:
    archive = json_parser.load(file, strict=False)

hash_archive = {}
for hash in archive:
    if archive[hash] in hash_archive:
        print("There are 2 saves with the same sha256. If it is not a mistake, open an issue immediately")
        exit()
    hash_archive[archive[hash].upper()] = hash

if not input("../archive/ will be replaced! Are you sure? y/n:").lower().startswith("y"):
    print("Exiting...")
    exit()

if os.path.exists('../archive/'):
    shutil.rmtree('../archive/')

os.mkdir('../archive/')

for key, value in hash_archive.items():
    with open(f'../archive/{key}', 'w') as file:
        file.write(value)
    
