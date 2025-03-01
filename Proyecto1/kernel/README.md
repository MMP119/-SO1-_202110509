Compilar el modulo:
<br>
make

<br>
Cargarlo en el kernel:
<br>
sudo insmod sysinfo.ko

<br>
Leer las métricas:
<br>
cat /proc/sysinfo_202110509

<br>
Verificar si el módulo ya está cargado:
<br>
lsmod | grep sysinfo

<br>
Quitar el módulo:
<br>
sudo rmmod sysinfo

<br>

