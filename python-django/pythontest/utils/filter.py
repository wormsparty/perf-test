from django.core.paginator import Paginator
from django.db.models import Q
from typing import List
import inflection


operator_mapping = {
    'startsWith': 'istartswith',
    'endsWith': 'iendswith',
    'contains': 'icontains',
    'notContains': 'icontains',
    'equals': 'iexact',
    'notEqual': 'iexact',
    'blank': 'exact',
    'notBlank': 'exact',
}

negative_operators = [
    'notContains',
    'notEqual',
    'notBlank',
]

class GridFilter():
    """Common filter classe for AG-Grid filtering and sorting"""

    def __init__(self, global_searchable_fields: List[str], id_field: str):
        self.global_searchable_fields = global_searchable_fields
        self.id_field = id_field


    def __get_filter(self, column: str, cond):
        """Apply a given filter, such as 'field contains value'"""
        filter_value = cond.get('filter')
        value = filter_value if filter_value else ''

        operator = operator_mapping[cond["type"]]

        if operator != '':
            return { f'{column}__{operator}': value }
        else:
            return { f'{column}': value }


    def apply(self, dataset, search_filters, sort_list, start: int, end: int, global_search: str):
        """Returns the list en entities, filtered and ordered according to the query parameters"""

        # Global filter
        if global_search:
            q = Q()

            for field in self.global_searchable_fields:
                q = q | Q(**{f'{field}__icontains': global_search})

            dataset = dataset.filter(q)

        # Filter
        for key in search_filters:
            column = inflection.underscore(key)
            value = search_filters[key]
            conditions = value.get("conditions")

            if conditions is not None:
                operator = value.get("operator")
                query = Q()

                for cond in conditions:
                    q = Q(**self.__get_filter(column, cond))

                    if cond["type"] in negative_operators:
                        q = ~q

                    if operator == "AND":
                        query = query & q
                    else:
                        query = query | q

                dataset = dataset.filter(query)
            else:
                q = Q(**self.__get_filter(column, value))

                if value["type"] in negative_operators:
                    q = ~q

                dataset = dataset.filter(q)

        # Sort
        if len(sort_list) > 0:
            for sort in sort_list:
                sort_direction = sort['sort']
                sort_column = sort['colId']

                # Special case: we will consider each table has a unique column ID,
                # and in our case we remap it in the backend
                if sort_column == 'id':
                    sort_column = self.id_field

                # GraphQL forces columns to be camelCase, however in Python our column names
                # are snake_case. Therefore we need to convert them.
                sort_column_snake_case = inflection.underscore(sort_column)

                if sort_direction == 'desc':
                    dataset = dataset.order_by(sort_column_snake_case)
                elif sort_direction == 'asc':
                    dataset = dataset.order_by(f'-{sort_column_snake_case}')
        else:
            dataset = dataset.order_by(self.id_field)

        page_size = end - start
        paginator = Paginator(dataset, page_size)
        page = paginator.get_page(start / page_size + 1)
        total = paginator.count

        return {
            "data": page,
            "total": total,
        }
