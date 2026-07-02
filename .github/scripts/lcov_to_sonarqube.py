#!/usr/bin/env python3
"""Convert LCOV coverage info to SonarQube generic coverage XML.

Reads lcov.info from the working directory and writes sonarqube-coverage.xml.
"""

import xml.etree.ElementTree as ET
from xml.dom import minidom
import sys


def parse_lcov(path: str) -> dict[str, list[tuple[str, bool]]]:
    files: dict[str, list[tuple[str, bool]]] = {}
    cur_file = None
    lines: list[tuple[str, bool]] = []

    for raw in open(path):
        line = raw.strip()
        if line.startswith("SF:"):
            cur_file = line[3:]
            lines = []
        elif line.startswith("DA:"):
            parts = line[3:].split(",")
            lines.append((parts[0], int(parts[1]) > 0))
        elif line == "end_of_record" and cur_file:
            files[cur_file] = lines
            cur_file = None
            lines = []

    return files


def build_xml(files: dict[str, list[tuple[str, bool]]]) -> str:
    root = ET.Element("coverage", version="1")

    for path, cov_lines in sorted(files.items()):
        fe = ET.SubElement(root, "file", path=path)
        for num, covered in cov_lines:
            ET.SubElement(fe, "lineToCover", lineNumber=num, covered=str(covered).lower())

    rough_string = ET.tostring(root)
    return minidom.parseString(rough_string).toprettyxml(indent="  ")


def main() -> None:
    input_path = "lcov.info"
    output_path = "sonarqube-coverage.xml"

    try:
        files = parse_lcov(input_path)
    except FileNotFoundError:
        print(f"Error: {input_path} not found", file=sys.stderr)
        sys.exit(1)

    xml = build_xml(files)
    with open(output_path, "w") as f:
        f.write(xml)

    print(f"Converted {input_path} -> {output_path} ({len(files)} files)")


if __name__ == "__main__":
    main()
