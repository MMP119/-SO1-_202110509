#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/sched.h>
#include <linux/mm.h>
#include <linux/smp.h>         // Para obtener número de CPUs
#include <linux/timekeeping.h> // Para obtener el tiempo desde el arranque
#include <linux/kernel_stat.h> // Para obtener tiempos de CPU
#include <linux/jiffies.h>     // Para calcular el tiempo
#include <linux/cpufreq.h>     // Para obtener la frecuencia del CPU
#include <linux/delay.h>       // Para msleep

#define FILE_NAME "sysinfo_202110509"

// ✅ Función para obtener la memoria del sistema
static void get_memory_info(struct seq_file *m) {
    struct sysinfo i;
    si_meminfo(&i);

    unsigned long total_ram = (i.totalram * i.mem_unit) / 1024;  // Convertir a KB
    unsigned long free_ram = (i.freeram * i.mem_unit) / 1024;
    unsigned long used_ram = total_ram - free_ram;

    seq_printf(m, "\t\"Memory\": {\n");
    seq_printf(m, "\t\t\"total_ram\": %lu KB,\n", total_ram);
    seq_printf(m, "\t\t\"free_ram\": %lu KB,\n", free_ram);
    seq_printf(m, "\t\t\"used_ram\": %lu KB\n", used_ram);
    seq_printf(m, "\t},\n");
}

// ✅ Función para obtener el uso de CPU
static void get_cpu_usage(struct seq_file *m) {
    unsigned long long user1 = 0, nice1 = 0, system1 = 0, idle1 = 0;
    unsigned long long user2 = 0, nice2 = 0, system2 = 0, idle2 = 0;
    unsigned int cpu;

    // Primera lectura de los tiempos de CPU
    for_each_online_cpu(cpu) {
        user1 += kcpustat_cpu(cpu).cpustat[CPUTIME_USER];
        nice1 += kcpustat_cpu(cpu).cpustat[CPUTIME_NICE];
        system1 += kcpustat_cpu(cpu).cpustat[CPUTIME_SYSTEM];
        idle1 += kcpustat_cpu(cpu).cpustat[CPUTIME_IDLE];
    }

    // Esperar 1 segundo para obtener una segunda muestra
    msleep(1000);

    // Segunda lectura de los tiempos de CPU
    for_each_online_cpu(cpu) {
        user2 += kcpustat_cpu(cpu).cpustat[CPUTIME_USER];
        nice2 += kcpustat_cpu(cpu).cpustat[CPUTIME_NICE];
        system2 += kcpustat_cpu(cpu).cpustat[CPUTIME_SYSTEM];
        idle2 += kcpustat_cpu(cpu).cpustat[CPUTIME_IDLE];
    }

    // Calcular las diferencias
    unsigned long long user = user2 - user1;
    unsigned long long nice = nice2 - nice1;
    unsigned long long system = system2 - system1;
    unsigned long long idle = idle2 - idle1;

    // Calcular el tiempo total de CPU
    unsigned long long total_time = user + nice + system;
    unsigned long long total_cpu_time = total_time + idle;

    // Calcular el uso de la CPU en porcentaje
    unsigned long long cpu_usage = (total_cpu_time > 0) ? (total_time * 100) / total_cpu_time : 0;

    seq_printf(m, "\t\"CPU_usage\": %llu%%\n", cpu_usage);
}

// ✅ Función que se ejecuta cuando se lee el archivo /proc/sysinfo
static int sysinfo_proc_show(struct seq_file *m, void *v) {
    seq_printf(m, "{\n");
    get_memory_info(m);
    get_cpu_usage(m);
    seq_printf(m, "}\n");
    return 0;
}

// ✅ Función que se ejecuta al abrir el archivo en /proc
static int sysinfo_proc_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_proc_show, NULL);
}

// ✅ Definir las operaciones del archivo en /proc
static const struct proc_ops sysinfo_proc_ops = {
    .proc_open = sysinfo_proc_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

// ✅ Función que se ejecuta al cargar el módulo
static int __init sysinfo_module_init(void) {
    proc_create(FILE_NAME, 0, NULL, &sysinfo_proc_ops);
    printk(KERN_INFO "Módulo sysinfo cargado en /proc/%s\n", FILE_NAME);
    return 0;
}

// ✅ Función que se ejecuta al descargar el módulo
static void __exit sysinfo_module_exit(void) {
    remove_proc_entry(FILE_NAME, NULL);
    printk(KERN_INFO "Módulo sysinfo eliminado de /proc/%s\n", FILE_NAME);
}

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Mario");
MODULE_DESCRIPTION("Módulo de kernel para capturar información del sistema en /proc");
MODULE_VERSION("1.0");

module_init(sysinfo_module_init);
module_exit(sysinfo_module_exit);