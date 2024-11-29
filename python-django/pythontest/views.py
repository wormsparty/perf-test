import json

from rest_framework import serializers
from rest_framework.response import Response
from rest_framework.views import APIView

from .models import Entity
from .utils.filter import grid_filter


global_searchable_fields = [
    'colonne_1',
    'colonne_2',
]

class EntitySerializer(serializers.ModelSerializer):
    class Meta:
        model = Entity
        fields = ["id", "colonne_1", "colonne_2"]


class AtlasApiView(APIView):
    def post(self, request, *args, **kwargs):
        """Returns the list en entities, filtered and ordered according to the query parameters"""

        body = json.loads(request.body.decode('UTF-8'))
        entities_with_total = grid_filter(Entity.objects.all(), body, global_searchable_fields)
        serialized_entities = EntitySerializer(entities_with_total.get['data'], many=True)

        return Response({
            'data': serialized_entities,
            'total': entities_with_total.get('total')
        })
