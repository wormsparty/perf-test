from typing import Dict, List, Optional
from pydantic import BaseModel

class EntityResult(BaseModel):
    id: int
    colonne_1: str
    colonne_2: str

    class Config:
        from_attributes = True


class LoginResponse(BaseModel):
    username: str


class EntityResponse(BaseModel):
    data: List[EntityResult]
    total: int


class FilterDetail(BaseModel):
    filterType: str
    type: str
    filter: str


class Sort(BaseModel):
    sort: str
    colId: str


class GridQuery(BaseModel):
    filter: Optional[Dict[str, FilterDetail]]
    sort: Optional[List[Sort]]
    start: int
    end: int
    globalSearch: Optional[str]