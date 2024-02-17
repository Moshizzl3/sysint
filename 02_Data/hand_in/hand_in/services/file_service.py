import pandas as pd
import xml.etree.ElementTree as ET
import yaml
from pydantic import BaseModel
import json
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent
PATH_TO_FILES = BASE_DIR / "static" / "files" / "pokemon"


class PokemonDTO(BaseModel):
    """DTO for a Pokemon"""

    name: str
    level: int
    elements: list[str]


def read_json_file() -> PokemonDTO:
    """Read json file, and return PokemonDTO."""

    with open(f"{PATH_TO_FILES}.json", "r") as file:
        data = json.load(file)

    return PokemonDTO(**data)


def read_csv_file() -> PokemonDTO:
    """Read csv file, and return PokemonDTO."""

    data = pd.read_csv(f"{PATH_TO_FILES}.csv").to_dict(orient="records")
    record = data[0]
    record["elements"] = record["elements"].split()

    return PokemonDTO(**record)


def read_txt_file() -> PokemonDTO:
    """Read txt file, and return PokemonDTO."""

    with open(f"{PATH_TO_FILES}.txt", "r") as file:
        lines = file.readlines()
    data = lines[1].split()

    name = data[0]
    level = int(data[1])
    elements = data[2:]

    return PokemonDTO(name=name, level=level, elements=elements)


def read_xml_file() -> PokemonDTO:
    """Read xml file, and return PokemonDTO."""

    # Parse file
    tree = ET.parse(f"{PATH_TO_FILES}.xml")
    root = tree.getroot()

    # Extract data
    name = root.find("name").text
    level = int(root.find("level").text)
    elements = [elem.text for elem in root.find("elements").findall("element")]

    return PokemonDTO(name=name, level=level, elements=elements)


def read_yaml_file() -> PokemonDTO:
    """Read json file, and return PokemonDTO."""

    with open(f"{PATH_TO_FILES}.yaml", "r") as file:
        data = yaml.safe_load(file)

    return PokemonDTO(**data)
