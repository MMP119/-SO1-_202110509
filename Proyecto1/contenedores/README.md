Iniciar el script:
<br>
./create_containers.sh
<br>
<br>

Eliminar todos los contenedores:
<br>
docker rm -f $(docker ps -aq) 
<br>
<br>

Detener solo los contenedores de estrés y no el log:
<br>
docker rm -f $(docker ps -aq --filter "name=container_")
<br>
<br>

AUTOMATIZACION CON CRONJOB
<br>
crontab -e
<br>
Ruta al final del archivo:
<br>
* * * * * /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh
<br>
Para cada 30 segundos:
<br>
* * * * * sleep 30; /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh



* * * * * /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh
* * * * * sleep 30; /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh






{
	"Memory":{
		"total_ram": 8012388,
		"free_ram": 636120,
		"used_ram": 7376268
	},
	"Processes":[
		{
			"pid": 32845,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id 0010109d96f447dd90b699c4859c423cc2defc8e9504de67d8dd827290071fe9 -address /run/containerd/containerd.sock ",
			"vsz": 1240908,
			"rss": 15588,
			"memoryUsage": 0.19,
			"cpuUsage": 0.19
		},
		{
			"pid": 32920,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id 4302fab4b52aa1d95c06a8cd27ca3136170c9e320fe3efcfed2671906b62b324 -address /run/containerd/containerd.sock ",
			"vsz": 1238976,
			"rss": 14476,
			"memoryUsage": 0.18,
			"cpuUsage": 0.14
		},
		{
			"pid": 32988,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id 48861cad0a521f177be69d71fdc2f963b1c0b4396d1c978cc3a25c98b06114d4 -address /run/containerd/containerd.sock ",
			"vsz": 1238976,
			"rss": 14624,
			"memoryUsage": 0.18,
			"cpuUsage": 0.19
		},
		{
			"pid": 33053,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id 76bfe7d7d0f56ace9cf4007c012166fd7183cb2218e428d766d80c60f8260e71 -address /run/containerd/containerd.sock ",
			"vsz": 1238720,
			"rss": 14624,
			"memoryUsage": 0.18,
			"cpuUsage": 0.21
		},
		{
			"pid": 33119,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id 4c69255aa114513235061528de0fc20aa8c4cbc90d2ac01c42122f565bf6a487 -address /run/containerd/containerd.sock ",
			"vsz": 1238976,
			"rss": 14492,
			"memoryUsage": 0.18,
			"cpuUsage": 0.17
		},
		{
			"pid": 33185,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id c23a931b431d2949ac28e80c4c1ea8f876c14dab377a819350bee05d50239943 -address /run/containerd/containerd.sock ",
			"vsz": 1238976,
			"rss": 14120,
			"memoryUsage": 0.17,
			"cpuUsage": 0.17
		},
		{
			"pid": 33252,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id b90620b5a548f44f531ac63b172a8492ad0c0859633213a932c76a48ef1fa1f4 -address /run/containerd/containerd.sock ",
			"vsz": 1238720,
			"rss": 14476,
			"memoryUsage": 0.18,
			"cpuUsage": 0.14
		},
		{
			"pid": 33318,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id 380881f7e586432327f8a7af735d6e4899bdd0a2103dc92443adbeced3b4cba3 -address /run/containerd/containerd.sock ",
			"vsz": 1238720,
			"rss": 14692,
			"memoryUsage": 0.18,
			"cpuUsage": 0.21
		},
		{
			"pid": 33385,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id f289f3108af378fa5d60b81f425ade13dc07f7e953bb4b71dfcbc79214988daa -address /run/containerd/containerd.sock ",
			"vsz": 1238720,
			"rss": 14464,
			"memoryUsage": 0.18,
			"cpuUsage": 0.19
		},
		{
			"pid": 33450,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id c081b321c6569da8f0274416263b710f3716c9e96e30313b6fa0a68891a5351b -address /run/containerd/containerd.sock ",
			"vsz": 1238976,
			"rss": 14680,
			"memoryUsage": 0.18,
			"cpuUsage": 0.26
		},
		{
			"pid": 33517,
			"name": "containerd-shim",
			"cmdLine": "/usr/bin/containerd-shim-runc-v2 -namespace moby -id f8a754836e073665ca4007dcc87f5c328215f00a9f900b07fdf9fe7ea3bd1ede -address /run/containerd/containerd.sock ",
			"vsz": 1238720,
			"rss": 14168,
			"memoryUsage": 0.17,
			"cpuUsage": 0.19
		}
	]
}