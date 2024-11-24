import json

from django.http import Http404, JsonResponse

from .models import Entity
from .utils.filter import grid_filter


global_searchable_fields = [
    'colonne_1',
    'colonne_2',
]


def api_list(request):
    """Returns the list en entities, filtered and ordered according to the query parameters"""

    if request.method != 'POST':
        raise Http404()

    body = json.loads(request.body.decode('UTF-8'))
    entities = grid_filter(Entity.objects.all(), body, global_searchable_fields)

    return JsonResponse({
        'data': [{
            "id": entity.id,
            "colonne_1": entity.colonne_1,
            "colonne_2": entity.colonne_2,
        } for entity in entities.get('data')],
        'total': entities.get('total')
    })
