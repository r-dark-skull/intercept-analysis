FROM python:3.13-bookworm

WORKDIR /app
COPY requirements.txt .
RUN python3 -m pip install -r requirements.txt

COPY app.py .

EXPOSE 1337
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port" ,"1337", "--workers", "10"]