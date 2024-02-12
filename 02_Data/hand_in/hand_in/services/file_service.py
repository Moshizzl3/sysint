import pandas as pd
import xml.etree.ElementTree as ET
import yaml
from fastapi import UploadFile
from io import StringIO
from uuid import uuid4


async def read_file(file: UploadFile) -> pd.DataFrame | None:
    if file.filename is None:
        return None

    content = await file.read()

    if file.filename.endswith(".csv"):
        return pd.read_csv(StringIO(content.decode()))
    elif file.filename.endswith(".txt"):
        return pd.read_csv(StringIO(content.decode()), sep=" ")
    elif file.filename.endswith(".json"):
        return pd.read_json(StringIO(content.decode()), lines=True)
    elif file.filename.endswith(".xml"):
        return pd.read_xml(StringIO(content.decode()))
    elif file.filename.endswith(".yaml"):
        return pd.json_normalize(yaml.safe_load(StringIO(content.decode())))
    else:
        raise ValueError("Not a valid input.")


async def convert_to_format(df: pd.DataFrame, format_type: str, file_path: str) -> str:
    """Function to convert DataFrame to various formats and return as string."""

    if format_type == "csv":
        return df.to_csv(path_or_buf=file_path, index=False)
    elif format_type == "txt":
        return df.to_csv(path_or_buf=file_path, sep="\t", index=False)
    elif format_type == "json":
        return df.to_json(path_or_buf=file_path, orient="records", lines=False)
    elif format_type == "xml":
        return df.to_xml()
    elif format_type == "yaml":
        dict_data = df.to_dict(orient="records")
        with open("output.yaml", "w") as file:
            return yaml.dump(dict_data, file)
    else:
        raise ValueError("Not a valid 'to' input.")
