from django.contrib.gis.db import models
from django.db import connection
from django.conf import settings
from django.contrib.gis.geos import Point


user = settings.AUTH_USER_MODEL


class GarbageNodes(models.Model):
    location = models.CharField(max_length=100, blank=True, default="")
    amount = models.IntegerField()
    instance = models.DateTimeField(auto_now_add=True)
    geolocation = models.PointField(geography=True, default=Point(0.0, 0.0))

    class Meta:
        ordering = ["amount"]

    def __str__(self):
        return self.location


def my_custom_sql():
    with connection.cursor() as cursor:
        cursor.execute("SELECT * FROM nodes_garbagenodes;")
        row = cursor.fetchall()

    return row


from julia.api import Julia
jl = Julia(compiled_modules=False)

from julia import Main
Main.include('./nodes/genetic.jl')
