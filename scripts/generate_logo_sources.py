#!/usr/bin/env python3
import base64
import os
from pathlib import Path

def convert_to_base64(image_path):
    with open(image_path, 'rb') as image_file:
        return base64.b64encode(image_file.read()).decode('utf-8')

def main():
    logos_dir = Path('logos')
    target_brands = ['canon', 'fujifilm', 'nikon', 'panasonic', 'sony', 'iphone']

    # Create src/logos directory if it doesn't exist
    src_logos_dir = Path('src/logos')
    src_logos_dir.mkdir(exist_ok=True)

    for brand in target_brands:
        image_path = logos_dir / f'{brand}.png'
        if not image_path.exists():
            print(f"Warning: {image_path} does not exist")
            continue

        base64_str = convert_to_base64(image_path)
        output_path = src_logos_dir / f'{brand}.base64'

        with open(output_path, 'w') as f:
            f.write(base64_str)
        print(f"Generated {brand}.base64 in src/logos")

if __name__ == '__main__':
    main()