from django.urls import path,include
from rest_framework import routers
from .views import NodeViewset,OptimalRoute,OptimalPlaces

router = routers.DefaultRouter()
router.register("garbagenode", NodeViewset)
# router.register("test", hello_world)



urlpatterns = [
    path('', include(router.urls)),
    path('OptimalRoute/', OptimalRoute),
    path('OptimalPlaces/', OptimalPlaces),
]