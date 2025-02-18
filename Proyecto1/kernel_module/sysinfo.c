#include <linux/module.h>     
#include <linux/proc_fs.h>    
#include <linux/seq_file.h>   
#include <linux/sched/signal.h> 
#include <linux/mm.h>         
#include <linux/sched.h>      

#define FILENAME "sysinfo_202110509"

// Función que obtiene la información del sistema
static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo i;
    si_meminfo(&i);

    // Obtener uso de CPU
    // Obtener uso de CPU en porcentaje
    unsigned long total_time = 0;
    unsigned long cpu_percentage = 0;
    struct task_struct *task;
    
    for_each_process(task) {
        total_time += task->se.sum_exec_runtime;
    }

    cpu_percentage = (total_time * 100) / (i.totalram * i.mem_unit);

    // Escribir la info general del sistema en JSON
    seq_printf(m, "{\n");
    seq_printf(m, "  \"RAM_total\": \"%lu MB\",\n", i.totalram * i.mem_unit / 1024 / 1024);
    seq_printf(m, "  \"RAM_libre\": \"%lu MB\",\n", (i.freeram + i.bufferram) * i.mem_unit / 1024 / 1024);
    seq_printf(m, "  \"RAM_en_uso\": \"%lu MB\",\n",(i.totalram - i.freeram - i.bufferram) * i.mem_unit / 1024 / 1024);
    seq_printf(m, "  \"CPU_usage\": \"%lu\",\n", cpu_percentage);
    
    // -------------------------
    // Capturar procesos de contenedores
    // -------------------------
    seq_printf(m, "  \"containers\": [\n");
    
    int first = 1;
    for_each_process(task) {
        if (task->flags & PF_KTHREAD) continue; // Omitir procesos del kernel
        if (!task->mm) continue;  // Omitir procesos sin espacio de memoria

        // Filtrar procesos relacionados con contenedores
        if (strstr(task->comm, "docker") || strstr(task->comm, "containerd")) {
            if (!first) seq_printf(m, ",\n");
            first = 0;

            seq_printf(m, "    {\n");
            seq_printf(m, "      \"PID\": \"%d\",\n", task->pid);
            seq_printf(m, "      \"Nombre\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"CPU\": \"%lu\",\n", task->se.sum_exec_runtime);
            seq_printf(m, "      \"RAM\": \"%lu KB\"\n", get_mm_rss(task->mm) * 4);
            seq_printf(m, "    }");
        }
    }
    seq_printf(m, "\n  ]\n");

    seq_printf(m, "}\n");
    return 0;
}

// Función que se ejecuta cuando se lee el archivo /proc/sysinfo
static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

// Definir las operaciones del archivo en /proc
static const struct proc_ops sysinfo_fops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

// Función que se ejecuta al cargar el módulo
static int __init sysinfo_init(void) {
    proc_create(FILENAME, 0, NULL, &sysinfo_fops);
    printk(KERN_INFO "Módulo sysinfo cargado en /proc/%s\n", FILENAME);
    return 0;
}

// Función que se ejecuta al descargar el módulo
static void __exit sysinfo_exit(void) {
    remove_proc_entry(FILENAME, NULL);
    printk(KERN_INFO "Módulo sysinfo eliminado de /proc/%s\n", FILENAME);
}

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Tu Nombre");
MODULE_DESCRIPTION("Módulo de Kernel para obtener información del sistema");

module_init(sysinfo_init);
module_exit(sysinfo_exit);
