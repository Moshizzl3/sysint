from fastapi import (
    APIRouter,
    UploadFile,
    File,
    HTTPException,
    BackgroundTasks,
)
from hand_in.services.file_service import read_file, convert_to_format
from fastapi.responses import FileResponse
from uuid import uuid4
import os
import tempfile

router = APIRouter()


def remove_file(file_path: str) -> None:
    """Remove file, if the file exist."""

    if os.path.exists(file_path):
        os.remove(file_path)


@router.post("/csv")
async def convert_to_csv(
    back_ground_task: BackgroundTasks, file: UploadFile = File(...)
) -> FileResponse:
    """
    Convert file to csv, and return the file.
    The file is stored in temp folder on the server,
    and then deleted through the background task.
    """

    df = await read_file(file)
    if df is None:
        raise HTTPException(status_code=400, detail="File format not supported")

    # Generate a unique temporary file path
    temp_dir = tempfile.gettempdir()
    temp_file_name = f"{uuid4()}.csv"
    temp_file_path = os.path.join(temp_dir, temp_file_name)

    await convert_to_format(df=df, format_type="csv", file_path=temp_file_path)

    # Add background task to delete the file.
    back_ground_task.add_task(remove_file, temp_file_path)

    return FileResponse(temp_file_path, filename="converted.csv", media_type="text/csv")


# @router.post("/txt")
# async def convert_to_txt(file: UploadFile = File(...)) -> Response:
#     df = await read_file(file)
#     if df is None:
#         raise HTTPException(status_code=400, detail="File format not supported")

#     txt_content = await convert_to_format(df, "txt")
#     return Response(content=txt_content, media_type="text")


# @router.post("/json")
# async def convert_to_json(file: UploadFile = File(...)) -> JSONResponse:
#     df = await read_file(file)
#     if df is None:
#         raise HTTPException(status_code=400, detail="File format not supported")

#     json_content = await convert_to_format(df, "json")
#     json_object = json.loads(json_content)
#     return JSONResponse(content=json_object)


# @router.post("/xml")
# async def convert_to_xml(file: UploadFile = File(...)) -> str:
#     df = await read_file(file)
#     if df is None:
#         raise HTTPException(status_code=400, detail="File format not supported")
#     return df.to_xml()


# @router.post("/yaml")
# async def convert_to_yaml(file: UploadFile = File(...)) -> str:
#     df = await read_file(file)
#     if df is None:
#         raise HTTPException(status_code=400, detail="File format not supported")
#     dict_data = df.to_dict(orient="records")
#     return yaml.dump(dict_data)
