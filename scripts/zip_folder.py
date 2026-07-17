#!/usr/bin/env python3
import sys
import os
import zipfile


def main():
    if len(sys.argv) < 3:
        print("Usage: zip_folder.py <source_dir> <output_zip>", file=sys.stderr)
        sys.exit(1)

    source_dir = sys.argv[1]
    output_zip = sys.argv[2]

    # Ensure parent directory of target zip exists
    output_dir = os.path.dirname(os.path.abspath(output_zip))
    if output_dir:
        os.makedirs(output_dir, exist_ok=True)

    with zipfile.ZipFile(output_zip, "w", zipfile.ZIP_DEFLATED) as zipf:
        for root, _, files in os.walk(source_dir):
            for file in files:
                file_path = os.path.join(root, file)
                # Store paths relative to the source directory
                arcname = os.path.relpath(file_path, source_dir)
                zipf.write(file_path, arcname)


if __name__ == "__main__":
    main()
