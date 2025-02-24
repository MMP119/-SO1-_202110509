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



{"StreamConfig":{},"State":{"Running":true,"Paused":false,"Restarting":false,"OOMKilled":false,"RemovalInProgress":false,"Dead":false,"Pid":43811,"ExitCode":0,"Error":"","StartedAt":"2025-02-24T20:24:41.32947465Z","FinishedAt":"0001-01-01T00:00:00Z","Health":null},"ID":"43f0f7a7161b170c4666003296a36c6faae295016849ff5416d67a524e17703c","Created":"2025-02-24T20:24:38.420859131Z","Managed":false,"Path":"stress","Args":["--hdd","1","--hdd-bytes","128M"],"Config":{"Hostname":"43f0f7a7161b","Domainname":"","User":"","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin","STRESS_VERSION=1.0.4"],"Cmd":["stress","--hdd","1","--hdd-bytes","128M"],"Image":"containerstack/alpine-stress","Volumes":null,"WorkingDir":"","Entrypoint":null,"OnBuild":null,"Labels":{"maintainer":"Remon Lam \u003cremon@containerstack.io\u003e"}},"Image":"sha256:759fa7c836fbaa3dad4a3ee02c8ac19afccde16aff399154738b074d294be83a","ImageManifest":null,"NetworkSettings":{"Bridge":"","SandboxID":"35d1d50d61587e032227f2c14b55b17f17b7091dc4cb9631c2f1866c5c4d08eb","SandboxKey":"/var/run/docker/netns/35d1d50d6158","HairpinMode":false,"LinkLocalIPv6Address":"","LinkLocalIPv6PrefixLen":0,"Networks":{"bridge":{"IPAMConfig":null,"Links":null,"Aliases":null,"MacAddress":"02:42:ac:11:00:0c","NetworkID":"42793c20b1f284cd01e6d68d4158b1487d7db57b36bf9bd71af0fee742895160","EndpointID":"09f534604013213aafdddceca3c0e6641861fbf52c04aec12c83cf4a234dfb44","Gateway":"172.17.0.1","IPAddress":"172.17.0.12","IPPrefixLen":16,"IPv6Gateway":"","GlobalIPv6Address":"","GlobalIPv6PrefixLen":0,"DriverOpts":null,"DNSNames":null,"IPAMOperational":false,"DesiredMacAddress":""}},"Service":null,"Ports":{},"SecondaryIPAddresses":null,"SecondaryIPv6Addresses":null,"HasSwarmEndpoint":false},"LogPath":"/var/lib/docker/containers/43f0f7a7161b170c4666003296a36c6faae295016849ff5416d67a524e17703c/43f0f7a7161b170c4666003296a36c6faae295016849ff5416d67a524e17703c-json.log","Name":"/container_1740428678337","Driver":"overlay2","OS":"linux","RestartCount":0,"HasBeenStartedBefore":true,"HasBeenManuallyStopped":false,"MountPoints":{},"SecretReferences":null,"ConfigReferences":null,"MountLabel":"","ProcessLabel":"","AppArmorProfile":"docker-default","SeccompProfile":"","NoNewPrivileges":false,"HostnamePath":"/var/lib/docker/containers/43f0f7a7161b170c4666003296a36c6faae295016849ff5416d67a524e17703c/hostname","HostsPath":"/var/lib/docker/containers/43f0f7a7161b170c4666003296a36c6faae295016849ff5416d67a524e17703c/hosts","ShmPath":"","ResolvConfPath":"/var/lib/docker/containers/43f0f7a7161b170c4666003296a36c6faae295016849ff5416d67a524e17703c/resolv.conf","LocalLogCacheMeta":{"HaveNotifyEnabled":false}}