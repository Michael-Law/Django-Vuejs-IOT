from django.urls import path, include
from rest_framework.urlpatterns import format_suffix_patterns
from snippets import views
from rest_framework_jwt.views import obtain_jwt_token, refresh_jwt_token
from rest_framework_simplejwt import views as jwt_views

# API endpoints
urlpatterns = format_suffix_patterns(
    [
        path("", views.api_root),
        path("snippets/", views.SnippetList.as_view(), name="snippet-list"),
        path(
            "snippets/<int:pk>/", views.SnippetDetail.as_view(), name="snippet-detail"
        ),
        path(
            "snippets/<int:pk>/highlight/",
            views.SnippetHighlight.as_view(),
            name="snippet-highlight",
        ),
        path("users/", views.UserList.as_view(), name="user-list"),
        path("users/<int:pk>/", views.UserDetail.as_view(), name="user-detail"),
        path(
            "api/token/",
            jwt_views.TokenObtainPairView.as_view(),
            name="token_obtain_pair",
        ),
        path(
            "api/token/refresh/",
            jwt_views.TokenRefreshView.as_view(),
            name="token_refresh",
        ),
    ]
)
