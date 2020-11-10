from django.urls import path, include
from rest_framework.urlpatterns import format_suffix_patterns
from snippets import views
from rest_framework_jwt.views import obtain_jwt_token, refresh_jwt_token

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
        url(r"^auth/obtain_token/", obtain_jwt_token),
        url(r"^auth/refresh_token/", refresh_jwt_token),
    ]
)
