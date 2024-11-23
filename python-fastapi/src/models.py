from sqlalchemy import Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base

Base = declarative_base()

class Entity(Base):
    __tablename__ = "entity"

    id        = Column(Integer, primary_key=True)
    colonne_1 = Column(String)
    colonne_2 = Column(String)
