from django.http import JsonResponse
from django.shortcuts import render

from .models import GarbageNodes
from rest_framework.viewsets import ModelViewSet
from .serializers import NodeSerializer


class NodeViewset(ModelViewSet):
    queryset = GarbageNodes.objects.all()
    serializer_class = NodeSerializer
