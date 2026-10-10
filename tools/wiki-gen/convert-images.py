#!/usr/bin/env python3
"""Turn the DDS textures wiki-gen copied out (icon sheets, zone minimaps) into PNG and JPEG
files under wiki/assets. Needs Pillow. Usage: convert-images.py <DDS folder> <wiki folder>"""

import json
import sys
from pathlib import Path

from PIL import Image

dds_dir = Path(sys.argv[1])
assets = Path(sys.argv[2]) / "assets"
for entry in json.loads((dds_dir / "convert.json").read_text()):
    target = assets / entry["target"]
    target.parent.mkdir(parents=True, exist_ok=True)
    image = Image.open(dds_dir / entry["dds"])
    if entry["format"] == "jpg":
        image.convert("RGB").save(target, "JPEG", quality=82, optimize=True)
    else:
        image.convert("RGBA").save(target, "PNG", optimize=True)
    print(target)
