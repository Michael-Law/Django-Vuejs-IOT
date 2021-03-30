from django.http import JsonResponse
from django.shortcuts import render
from django.db import connection
from rest_framework.decorators import api_view
from .models import GarbageNodes
from rest_framework.viewsets import ModelViewSet
from .serializers import NodeSerializer
from rest_framework.response import Response
import pandas as pd
import geopandas as gp
import pathlib
import requests


class NodeViewset(ModelViewSet):
    queryset = GarbageNodes.objects.all()
    serializer_class = NodeSerializer


@api_view()
def OptimalRoute(request):
    with connection.cursor() as cursor:
        cursor.execute(
            "SELECT id, location, amount, instance, ST_AsText(geolocation) FROM nodes_garbagenodes;")
        row = cursor.fetchall()
    columns = ["id", "Location", "amount", "instance", "Geolocation"]
    df = pd.DataFrame(row, columns=columns)
    NonOptDict = df.to_dict('records')

    """
    Julia Api backend for genetic algorithm, which will input the 
    dictionnary to be optimised.
    """
    from julia.api import Julia
    jl = Julia(compiled_modules=False)
    from julia import Main
    Main.include('./nodes/Genetic.jl')
    res, des = Main.initialMatrix(NonOptDict)

    return Response(res)


@api_view()
def OptimalPlaces(request):
    with connection.cursor() as cursor:
        cursor.execute(
            "SELECT id, location, amount, instance, ST_AsText(geolocation) FROM nodes_garbagenodes;")
        row = cursor.fetchall()
    columns = ["id", "Location", "amount", "instance", "Geolocation"]
    df = pd.DataFrame(row, columns=columns)
    NonOptDict = df.to_dict('records')

    """
    Julia Api backend for genetic algorithm, which will input the 
    dictionnary to be optimised.
    """
    from julia.api import Julia
    jl = Julia(compiled_modules=False)
    from julia import Main
    Main.include('./nodes/Genetic.jl')

    res, des = Main.initialMatrix(NonOptDict)

    return Response(des)


@api_view()
def NearestPlaces(request):
    with connection.cursor() as cursor:
        cursor.execute(
            "SELECT id, location, amount, instance, ST_AsText(geolocation) FROM nodes_garbagenodes;")
        row = cursor.fetchall()
    columns = ["id", "Location", "amount", "instance", "Geolocation"]
    df = pd.DataFrame(row, columns=columns)
    NonOptDict = df.to_dict('records')

    """
    Julia Api backend for genetic algorithm, which will input the
    dictionnary to be optimised.
    """
    from julia.api import Julia
    jl = Julia(compiled_modules=False)
    from julia import Main

    # data = request.data
    Main.include('./nodes/Genetic.jl')
    data = (57.48358652376267, -20.260487410613507)
    res = Main.binsearch(NonOptDict, data)
    return Response(res)


@api_view(['POST'])
def nearestnode(request):
    data = request.data
    url = '172.104.166.102:8080/transaction'
    feedback = requests.post(url, data=data)
    return feedback
