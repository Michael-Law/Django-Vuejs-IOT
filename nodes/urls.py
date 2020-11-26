from django.urls import path
from rest_framework import routers
from .views import NodeViewset

router = routers.DefaultRouter()
router.register("garbagenode", NodeViewset)

urlpatterns = router.urls
