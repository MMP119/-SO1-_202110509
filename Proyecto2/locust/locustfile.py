from locust import HttpUser, task, between
import random

class WeatherTestUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def send_weather_data(self):
        data = {
            "description": random.choice(["Moderado", "Grave", "Leve"]),
            "country": "GT",
            "weather": random.choice(["Lluvioso", "Nubloso", "Soleado"])
        }
        self.client.post("/input", json=data)
