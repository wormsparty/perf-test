from django.db.models import Q, Window, Count, QuerySet
from typing import List
from unidecode import unidecode

def grid_filter(qs: QuerySet, query, global_searchable_fields: List[str]):
    """Returns the list en entities, filtered and ordered according to the query parameters"""

    qs = qs.annotate(
        total=Window(
            expression=Count('*'),
            partition_by=None
        )
    )

    # Filter
    for column, field_filter in query['filter'].items():
        filter_value = unidecode(field_filter['filter'])
        unaccent_column = f"{column}__unaccent"

        match field_filter['type']:
            case 'equals':
                qs = qs.filter(**{f"{unaccent_column}": filter_value})
            case 'notEquals':
                qs = qs.filter(~Q(**{f"{unaccent_column}": filter_value}))
            case 'contains':
                qs = qs.filter(**{f"{unaccent_column}__icontains": filter_value})
            case 'notContains':
                qs = qs.filter(~Q(**{f"{unaccent_column}__icontains": filter_value}))
            case 'startsWith':
                qs = qs.filter(**{f"{unaccent_column}__istartswith": filter_value})
            case 'endsWith':
                qs = qs.filter(**{f"{unaccent_column}__iendswith": filter_value})
            case 'blank':
                qs = qs.filter(
                    Q(**{f"{column}": ""}) | Q(**{f"{column}__isnull": True})
                )
            case 'notBlank':
                qs = qs.filter(
                    ~(Q(**{f"{column}": ""}) | Q(**{f"{column}__isnull": True}))
                )

    # Global filter
    global_search = query['globalSearch']

    if global_search:
        global_search_normalized = unidecode(global_search)
        q = Q()

        for column in global_searchable_fields:
            unaccent_column = f"{column}__unaccent"
            q = q | Q(**{f'{unaccent_column}__icontains': global_search_normalized})

        qs = qs.filter(q)

    # Sort
    sort = query['sort']

    if sort:
        first_sort = sort[0]

        match first_sort['sort'].lower():
            case 'desc':
                qs = qs.order_by(first_sort['colId'])
            case 'asc':
                qs = qs.order_by(f'-{first_sort["colId"]}')
    else:
        qs = qs.order_by('id')

    qs = qs[query['start']:query['end']]

    data = list(qs)
    total = data[0].total if len(data) > 0 else 0

    return {
        "data": data,
        "total": total,
    }
