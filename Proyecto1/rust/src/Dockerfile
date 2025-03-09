# Usa una imagen base de Python
FROM python:3.9-slim

# Establece el directorio de trabajo
WORKDIR /app

#instala FastAPI y Uvicorn directamente
RUN pip install fastapi uvicorn

# Copia el código de la API al contenedor
COPY main.py .

# Crea el directorio para los logs y define el volumen
RUN mkdir -p /data
VOLUME ["/data"]

# Expone el puerto 8000
EXPOSE 8000

# Comando para iniciar el servidor
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
