from typing import List
from unidecode import unidecode
from sqlalchemy import or_, not_, func
from sqlalchemy.sql.elements import ColumnElement
from sqlalchemy.orm import Query
from . import schemas
from .schemas import EntityResult


def grid_filter(request: schemas.GridQuery, global_searchable_fields: List[str], query: Query) -> schemas.EntityResponse:
    model = query.column_descriptions[0]['entity']

    query = query.with_entities(
        func.count().over().label('total_count'),
        *[getattr(model, c.name) for c in model.__table__.columns]
    )

    # Filter
    for column_name, field_filter in request.filter.items():
        if not hasattr(model, column_name):
            raise ValueError(f"La colonne '{column_name}' n'existe pas dans le modèle '{model.__name__}'.")

        column: ColumnElement = getattr(model, column_name)
        unaccented_column = func.unaccent(column)
        search_value = unidecode(field_filter.filter) if field_filter.filter else ""

        match field_filter.type:
            case 'equals':
                query = query.filter(unaccented_column.eq(search_value))
            case 'notEquals':
                query = query.filter(not_(unaccented_column.eq(search_value)))
            case 'contains':
                query = query.filter(unaccented_column.ilike(f'%{search_value}%'))
            case 'notContains':
                query = query.filter(not_(unaccented_column.ilike(f'{search_value}%')))
            case 'startsWith':
                query = query.filter(unaccented_column.ilike(f'{search_value}%'))
            case 'endsWith':
                query = query.filter(unaccented_column.ilike(f'%{search_value}'))
            case 'blank':
                query = query.filter(or_(column == '', column.is_(None)))
            case 'notBlank':
                query = query.filter(not_(or_(column == '', column.is_(None))))

    # Global filter
    if request.globalSearch:
        conditions = []

        for column_name in global_searchable_fields:
            if not hasattr(model, column_name):
                raise ValueError(f"La colonne '{column_name}' n'existe pas dans le modèle '{model.__name__}'.")

            column = getattr(model, column_name)
            unaccented_column = func.unaccent(column)
            search_value = unidecode(request.globalSearch) if request.globalSearch else ""

            conditions.append(unaccented_column.ilike(f'%{search_value}%'))

        query = query.filter(or_(*conditions))

    # Sort
    if request.sort:
        sort = request.sort[0]

        if not hasattr(model, sort.colId):
            raise ValueError(f"La colonne '{sort.colId}' n'existe pas dans le modèle '{model.__name__}'.")

        column = getattr(model, sort.colId)

        match sort.sort.lower():
            case 'desc':
                query = query.order_by(column.desc())
            case 'asc':
                query = query.order_by(column.asc())
    else:
        query = query.order_by(getattr(model, 'id'))

    # Pagination
    query = query.offset(request.start).limit(request.end - request.start)

    # To debug query
    # print(str(query))

    results = query.all()
    total = results[0].total_count if results else 0

    data = [
        EntityResult.model_validate({
            column.name: getattr(row, column.name)
            for column in model.__table__.columns
        })
        for row in results
    ]

    return schemas.EntityResponse(data = data, total = total)