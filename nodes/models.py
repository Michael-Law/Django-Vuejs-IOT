from django.db import models


class GarbageNodes(models.Model):
    created = models.DateTimeField(auto_now_add=False)
    placename = models.CharField(max_length=100, blank=True, default="")
    amount = models.IntegerField()
    lon = models.FloatField()
    lat = models.FloatField()

    class Meta:
        ordering = ["amount"]

    def __str__(self):
        return self.placename