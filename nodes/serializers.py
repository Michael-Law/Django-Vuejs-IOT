from rest_framework.serializers import ModelSerializer
from .models import GarbageNodes

from django.contrib.auth.models import User


class NodeSerializer(ModelSerializer):
    class Meta:
        model = GarbageNodes
        fields = ["created", "placename", "amount", "lat", "lon"]
