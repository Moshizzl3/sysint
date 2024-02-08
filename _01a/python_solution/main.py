import argparse
import pandas as pd
import xml.etree.ElementTree as ET
import yaml


from bla import ClassA

hej = ClassA()


def read_file(file_path: str) -> pd.DataFrame | None:
    """Reads file based on extension."""

    if file_path.endswith(".csv"):
        return pd.read_csv(file_path)
    elif file_path.endswith(".txt"):
        return pd.read_csv(file_path, sep=" ")
    elif file_path.endswith(".json"):
        return pd.read_json(file_path, lines=True)
    elif file_path.endswith(".xml"):
        return pd.read_xml(file_path)
    elif file_path.endswith(".yaml"):
        return pd.json_normalize()

    return None


def convert_to_format(df: pd.DataFrame, format_type: str) -> None:
    """Function to convert DataFrame to various formats."""

    if format_type == "csv":
        df.to_csv("output.csv", index=False)
    elif format_type == ".txt":
        df.to_csv("output.txt", sep="\t", index=False)
    elif format_type == "json":
        df.to_json("output.json", orient="records", lines=True)
    elif format_type == "xml":
        df.to_xml("output.xml")
    elif format_type == "yaml":
        dict_data = df.to_dict(orient="records")
        with open("output.yaml", "w") as file:
            yaml.dump(dict_data, file)
    else:
        raise ValueError("Not a valid 'to' input.")


def main():
    parser = argparse.ArgumentParser(description="Convert file to specified format.")
    parser.add_argument("-f", "--file", required=True, help="Input file path")
    parser.add_argument(
        "-t",
        "--to",
        required=True,
        help="Format to convert to (text, xml, yaml, json, csv)",
    )
    args = parser.parse_args()

    df = read_file(args.file)
    if df is None:
        print("Not a valid input.")
        return

    try:
        convert_to_format(df, args.to.lower())
        print(f"file is saved in format: {args.to}")
    except ValueError as e:
        print(e)


if __name__ == "__main__":
    main()
