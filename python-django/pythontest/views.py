import json

from django.http import Http404, JsonResponse
from django.conf import settings

from .models import Entity
from .utils.filter import GridFilter


global_searchable_fields = [
    'colonne_1',
    'colonne_2',
]


def api_list(request):
    """Returns the list en entities, filtered and ordered according to the query parameters"""

    if request.method != 'POST':
        raise Http404()

    body = json.loads(request.body.decode('UTF-8'))

    search_filter = body.get('filter')
    sort = body.get('sort')
    start = int(body.get('start'))
    end = int(body.get('end'))
    global_search = body.get('globalSearch')

    grid_filter = GridFilter(global_searchable_fields, 'sharepoint_id')
    entities = grid_filter.apply(Entity.objects.all(), search_filter, sort, start, end, global_search)

    return JsonResponse({
        'data': [{
            "id": entity.id,
            "colonne_1": entity.colonne_1,
            "colonne_2": entity.colonne_2,
        } for entity in entities.get('data')],
        'total': entities.get('total')
    })
