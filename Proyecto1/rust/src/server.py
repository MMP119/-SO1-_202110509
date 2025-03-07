from fastapi import FastAPI
import json
import os
from datetime import datetime

app = FastAPI()

# asegurar que exista la carpeta de logs
os.makedirs("logs", exist_ok=True)

# archivo donde se guardarán las métricas
LOG_FILE = "logs/metrics.json"

@app.post("/logs")
async def receive_logs(data: dict):
    """ Recibe métricas del sistema y contenedores y las guarda en un archivo JSON """
    
    #agregar timestamp
    data["timestamp"] = datetime.now().isoformat()
    
    #cargar datos previos si existen
    if os.path.exists(LOG_FILE):
        with open(LOG_FILE, "r") as f:
            try:
                logs = json.load(f)
            except json.JSONDecodeError:
                logs = []
    else:
        logs = []
    
    #agregar la nueva entrada
    logs.append(data)
    
    #guardar en el archivo
    with open(LOG_FILE, "w") as f:
        json.dump(logs, f, indent=4)
    
    return {"message": "Logs recibidos correctamente"}


@app.get("/logs")
async def get_logs():
    """ Retorna todos los logs guardados """
    if os.path.exists(LOG_FILE):
        with open(LOG_FILE, "r") as f:
            try:
                logs = json.load(f)
                return logs
            except json.JSONDecodeError:
                return {"error": "No se pudo leer el archivo de logs"}
    
    return {"message": "No hay logs disponibles"}