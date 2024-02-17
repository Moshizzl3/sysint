from fastapi import (
    APIRouter,
)
from hand_in.services import file_service
from pydantic import BaseModel

router = APIRouter()


class DataResponseDTO(BaseModel):
    data: file_service.PokemonDTO


@router.get("/json")
async def get_json_file() -> DataResponseDTO:
    """Endpoint for getting json file data."""

    response_dto = file_service.read_json_file()

    return DataResponseDTO(data=response_dto)


@router.get("/csv")
async def get_csv_file() -> DataResponseDTO:
    """Endpoint for getting csv file data."""

    response_dto = file_service.read_csv_file()

    return DataResponseDTO(data=response_dto)


@router.get("/txt")
async def get_txt_file() -> DataResponseDTO:
    """Endpoint for getting txt file data."""

    response_dto = file_service.read_txt_file()

    return DataResponseDTO(data=response_dto)


@router.get("/xml")
async def get_xml_file() -> DataResponseDTO:
    """Endpoint for getting xml file data."""

    response_dto = file_service.read_xml_file()

    return DataResponseDTO(data=response_dto)


@router.get("/yaml")
async def get_yaml_file() -> DataResponseDTO:
    """Endpoint for getting yaml file data."""

    response_dto = file_service.read_yaml_file()

    return DataResponseDTO(data=response_dto)
