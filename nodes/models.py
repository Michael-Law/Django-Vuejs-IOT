from django.db import models
from django.conf import settings

user = settings.AUTH_USER_MODEL


class GarbageNodes(models.Model):
    # owner = models.ForeignKey(user, related_name="nodes", on_delete=models.CASCADE)
    created = models.DateTimeField(auto_now_add=False)
    placename = models.CharField(max_length=100, blank=True, default="")
    amount = models.IntegerField()
    lon = models.FloatField()
    lat = models.FloatField()

    class Meta:
        ordering = ["amount"]

    def __str__(self):
        return self.placename