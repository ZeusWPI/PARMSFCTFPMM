FROM python:alpine

WORKDIR /parmesan

COPY requirements.txt requirements.txt

RUN pip3 install -r requirements.txt

COPY main.py main.py

# CMD [ "python3", "-m", "flask", "--app", "main.py", "run", "--host=0.0.0.0" ]
CMD [ "gunicorn", "-w", "4", "-b", "0.0.0.0:5000", "--log-level", "debug", "main:app" ]
