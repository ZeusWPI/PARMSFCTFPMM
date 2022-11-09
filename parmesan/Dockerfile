FROM python:alpine

WORKDIR /parmesan

RUN pip3 install flask

COPY main.py main.py

CMD [ "python3", "-m", "flask", "--app", "main.py", "run", "--host=0.0.0.0" ]
