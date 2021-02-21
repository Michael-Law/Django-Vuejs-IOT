while true; do
  echo "Re-starting Django runserver"
  pipenv shell
  python manage.py runserver
  sleep 10
done
