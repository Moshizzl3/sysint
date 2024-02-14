from fastapi.routing import APIRouter

from hand_in.web.api import monitoring
from hand_in.web.api import file

api_router = APIRouter()
api_router.include_router(monitoring.router)
api_router.include_router(file.router, prefix="/file", tags=["file"])
