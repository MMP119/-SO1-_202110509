from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from datetime import datetime
import json
import os
from typing import List, Optional

app = FastAPI()

# Ruta donde se almacenarán los logs
LOGS_FILE = "/data/logs.json"

class ContainerLog(BaseModel):
    id: str
    fecha_creacion: datetime
    fecha_eliminacion: Optional[datetime] = None
    metric: Optional[str] = None

class LogEntry(BaseModel):
    timestamp: datetime
    memory_total: str
    memory_free: str
    memory_used: str
    cpu_usage: str
    cpu: List[ContainerLog] = []
    ram: List[ContainerLog] = []
    io: List[ContainerLog] = []
    disco: List[ContainerLog] = []

@app.post("/logs")
async def receive_logs(log: LogEntry):
    # Lee los logs existentes si existen
    logs = []
    if os.path.exists(LOGS_FILE):
        try:
            with open(LOGS_FILE, "r") as f:
                logs = json.load(f)
        except Exception as e:
            raise HTTPException(status_code=500, detail=f"Error al leer los logs: {e}")
    
    logs.append(log.dict())
    
    try:
        with open(LOGS_FILE, "w") as f:
            json.dump(logs, f, default=str, indent=4)
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error al guardar los logs: {e}")
    
    return {"status": "ok"}

@app.get("/logs")
async def get_logs():
    if os.path.exists(LOGS_FILE):
        try:
            with open(LOGS_FILE, "r") as f:
                logs = json.load(f)
            return logs
        except Exception as e:
            raise HTTPException(status_code=500, detail=f"Error al leer los logs: {e}")
    else:
        return []
    

@app.delete("/borrar")
async def delete_logs():
    if os.path.exists(LOGS_FILE):
        try:
            os.remove(LOGS_FILE)
            return {"status": "ok", "message": "Logs borrados"}
        except Exception as e:
            raise HTTPException(status_code=500, detail=f"Error al borrar logs: {e}")
    else:
        return {"status": "ok", "message": "No hay logs para borrar"}


@app.get("/")
async def root():
    return {"message": "Contenedor de Logs funcionando"}
