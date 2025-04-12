from locust import HttpUser, task, between
import random

class WeatherTestUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def send_weather_data(self):
        data = {
            "description": random.choice(["Moderado", "Grave", "Leve"," Muy Grave", "Extremo","Normal"]),
            "country": random.choice(["GT","SV","HN","NI","CR","PA","CO","VE","EC","PE","BR","PY","UY","AR"]),
            "weather": random.choice(["Lluvioso", "Nubloso", "Soleado"])
        }
        self.client.post("/input", json=data)
