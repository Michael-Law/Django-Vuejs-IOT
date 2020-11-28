from rest_framework import serializers
from rest_framework.permissions import IsAuthenticated
from django.db import models
from django.contrib.auth import authenticate
from django.contrib.auth.hashers import make_password  # Register serializer
from django.http import HttpResponse, JsonResponse
from django.contrib.auth import get_user_model
from .models import StaffProfile
import json

User = StaffProfile


class UserCreateSerializer(serializers.ModelSerializer):
    password = serializers.CharField(
        write_only=True, required=True, style={"input_type": "password"}
    )
    password2 = serializers.CharField(
        style={"input_type": "password"}, write_only=True, label="Confirm password"
    )

    class Meta:
        model = User
        fields = [
            "username",
            "email",
            "company",
            "password",
            "password2",
        ]
        extra_kwargs = {"password": {"write_only": True}}

    def create(self, validated_data):
        username = validated_data["username"]
        email = validated_data["email"]
        company = validated_data["company"]
        password = validated_data["password"]
        password2 = validated_data["password2"]
        if (
            email
            and User.objects.filter(email=email).exclude(username=username).exists()
        ):
            raise serializers.ValidationError(
                {"email": "Email addresses must be unique."}
            )
        if password != password2:
            raise serializers.ValidationError({"password": "The two passwords differ."})
        user = User(username=username, company=company, email=email)
        user.set_password(password)
        user.save()
        return user


class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = "__all__"
