from rest_framework import generics, permissions, mixins
from rest_framework.response import Response
from .serializer import UserCreateSerializer, UserSerializer
from django.contrib.auth.models import User  # Register


class RegisterApi(generics.GenericAPIView):
    serializer_class = UserCreateSerializer

    def post(self, request, *args, **kwargs):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        user = serializer.save()
        return Response(
            {
                "user": UserSerializer(
                    user, context=self.get_serializer_context()
                ).data,
                "message": "User Created Successfully.  Now perform Login to get your token",
            }
        )
