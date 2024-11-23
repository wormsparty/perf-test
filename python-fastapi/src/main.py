import os

from fastapi import FastAPI, Depends, Request
from fastapi.middleware.cors import CORSMiddleware

from dotenv import load_dotenv

from sqlalchemy.orm import Session
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

from . import schemas
from .filter import grid_filter
from .models import Entity
from .schemas import LoginResponse

# .env configuration
BASEDIR = os.path.abspath(os.path.dirname(__file__))
load_dotenv(os.path.join(BASEDIR, '..', '..', '.env'))

# DB setup
engine = create_engine(os.getenv("DATABASE_URL"))
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

# API setup
app = FastAPI()
cors_all = bool(os.getenv("CORS_ORIGIN_ALLOW_ALL"))
cors_origins = []

if not cors_all:
    cors_origins = (os.getenv("CORS_ALLOWED_ORIGINS") or "").split(',')

app.add_middleware(
    CORSMiddleware,
    allow_origins=cors_origins,
    allow_credentials=cors_all,
    allow_methods=["*"],
    allow_headers=["*"],
)

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


@app.post("/api/list", response_model=schemas.EntityResponse)
def api_list(request: schemas.GridQuery, db: Session = Depends(get_db)):
    global_searchable_fields = [
        'colonne_1',
        'colonne_2',
    ]

    return grid_filter(request, global_searchable_fields, db.query(Entity))

@app.post("/api/login", response_model=schemas.LoginResponse)
def api_login(request: Request):
    return LoginResponse(username=request.headers.get('Userid', 'VDL12345'))
