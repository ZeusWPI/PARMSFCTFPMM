FROM python:alpine

WORKDIR /rust

COPY requirements.txt requirements.txt

RUN pip3 install -r requirements.txt

COPY main.py main.py
COPY templates templates

# CMD [ "python3", "-m", "flask", "--app", "main.py", "run", "--host=0.0.0.0", "--port=8080"]
CMD [ "gunicorn", "-w", "4", "-b", "0.0.0.0:8080", "--log-level", "debug", "main:app" ]
