from django.contrib.gis.db import models
from django.conf import settings
from django.contrib.gis.geos import Point


user = settings.AUTH_USER_MODEL


class GarbageNodes(models.Model):
    location = models.CharField(max_length=100, blank=True, default="")
    amount = models.IntegerField()
    instance = models.DateTimeField(auto_now_add=True)
    geolocation = models.PointField(geography=True,srid=4326 ,default=Point(0.0, 0.0))

    class Meta:
        ordering = ["amount"]

    def __str__(self):
        return self.location



