# **PROYECTO 1**



826d9a39f0b94bb5780af5abb9a17a3608c28eec9d592595045a5b0fce764113
📦 Contenedor creado: container_1741324513839 (--cpu 1)
911f158986deabd41744b73ce5fd0e3bd5b39450b30decd5beba0c44ec6ce362
📦 Contenedor creado: container_1741324514380 (--vm 1 --vm-bytes 128M)
238ca46bbae60d9fc6111f17409d08b79c4f23ac3c91efa373f7e4dde2a75096
📦 Contenedor creado: container_1741324514678 (--io 1)
0ced2212817100980d75bf793437ccc846f1ae11329c2cd9f487a77d9c335b17
📦 Contenedor creado: container_1741324514999 (--hdd 1 --hdd-bytes 128M)
f070aa9ec3d5a610701080af2901e5b9c4757dd6cc2d7dfdb02333b76c223275
📦 Contenedor creado: container_1741324515271 (--hdd 1 --hdd-bytes 128M)
a6421a7660e0646ca65dc2883eb995b7b8ff9b67c235f14b714f5605eaddd6e2
📦 Contenedor creado: container_1741324515546 (--vm 1 --vm-bytes 128M)
e3d42db67b6a2bbe6f4b889d0e40de41ccd09f0446119989e012ac835299e5b5
📦 Contenedor creado: container_1741324515885 (--hdd 1 --hdd-bytes 128M)
f0d4ed86803f5e67e91e72086d1c9c60df627444ee056c141ca6eb5c78f630d1
📦 Contenedor creado: container_1741324516182 (--io 1)
f294c064d296f69b791cd640ce6a4a2fb330bea8e4435418569c40b2842b10c8
📦 Contenedor creado: container_1741324516762 (--io 1)
d8d23c47d1d0f8775cf884fe4bb4c30dbb0818a491e90bcace39077185bf4e11
📦 Contenedor creado: container_1741324517061 (--hdd 1 --hdd-bytes 128M)
✅ Se han creado 10 nuevos contenedores de estrés.





CONTAINER ID   IMAGE                          COMMAND                  CREATED              STATUS              PORTS                                       NAMES
f294c064d296   containerstack/alpine-stress   "stress --io 1"          About a minute ago   Up About a minute                                       container_1741324516762
a6421a7660e0   containerstack/alpine-stress   "stress --vm 1 --vm-…"   About a minute ago   Up About a minute                                       container_1741324515546
f070aa9ec3d5   containerstack/alpine-stress   "stress --hdd 1 --hd…"   About a minute ago   Up About a minute                                       container_1741324515271
826d9a39f0b9   containerstack/alpine-stress   "stress --cpu 1"         About a minute ago   Up About a minute                                       container_1741324513839
f3db32c4c428   logs_container                 "uvicorn server:app …"   About a minute ago   Up About a minute   0.0.0.0:8000->8000/tcp, :::8000->8000/tcp   logs_manager
