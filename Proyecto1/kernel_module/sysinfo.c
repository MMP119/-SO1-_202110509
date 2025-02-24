#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/sched.h>
#include <linux/cgroup.h>
#include <linux/string.h>
#include <linux/slab.h>
#include <linux/fs.h>
#include <linux/uaccess.h>
#include <linux/delay.h> // Para msleep

#define FILE_NAME "sysinfo_202110509"
#define MAX_CONTAINERS 11
#define CONTAINER_ID_LENGTH 12 // Longitud de los IDs en docker ps

// Estructura para almacenar información de un contenedor
struct container_info {
    char id[CONTAINER_ID_LENGTH + 1]; // +1 para el carácter nulo
    char name[64];
    pid_t pid;
    char memory_usage[16];
    char cpu_usage[16];
    char io_usage[16];
};

// Función para obtener el ID del contenedor desde el cgroup del proceso
static const char* get_container_id(struct task_struct *task) {
    struct cgroup *cgrp = task->cgroups->dfl_cgrp;
    if (cgrp && cgrp->kn) {
        // Verificar si el cgroup está bajo el directorio /docker/
        const char *cgroup_name = cgrp->kn->name;
        if (strstr(cgroup_name, "docker")) {
            // Eliminar el prefijo "docker-" si está presente
            const char *prefix = "docker-";
            if (strncmp(cgroup_name, prefix, strlen(prefix)) == 0) {
                cgroup_name += strlen(prefix);
            }
            // Eliminar el sufijo ".scope" si está presente
            const char *suffix = ".scope";
            if (strstr(cgroup_name, suffix)) {
                char *container_id = kstrdup(cgroup_name, GFP_KERNEL);
                if (container_id) {
                    char *suffix_pos = strstr(container_id, suffix);
                    if (suffix_pos) {
                        *suffix_pos = '\0'; // Truncar el sufijo
                    }
                    return container_id;
                }
            }
        }
    }
    return NULL; // No es un contenedor Docker
}

// Función para leer un archivo en el kernel
static int read_file(const char *path, char *buffer, size_t size) {
    struct file *file;
    char *kbuf;
    int ret = 0;

    file = filp_open(path, O_RDONLY, 0);
    if (IS_ERR(file)) {
        printk(KERN_ERR "Error: No se pudo abrir el archivo %s\n", path);
        return PTR_ERR(file);
    }

    kbuf = kmalloc(size, GFP_KERNEL);
    if (!kbuf) {
        ret = -ENOMEM;
        goto out;
    }

    ret = kernel_read(file, kbuf, size - 1, &file->f_pos);
    if (ret > 0) {
        kbuf[ret] = '\0'; // Asegurarnos de que esté terminado en NULL
        strncpy(buffer, kbuf, size);
    } else {
        printk(KERN_ERR "Error: No se pudo leer el archivo %s\n", path);
    }

    kfree(kbuf);
out:
    filp_close(file, NULL);
    return ret;
}

// Función para obtener el uso de memoria de un contenedor
static unsigned long get_memory_usage(const char *container_id) {
    char path[256];
    char buffer[64];
    unsigned long memory_usage = 0;

    snprintf(path, sizeof(path), "/sys/fs/cgroup/system.slice/docker-%s.scope/memory.current", container_id);
    printk(KERN_INFO "Leyendo memoria desde: %s\n", path);

    if (read_file(path, buffer, sizeof(buffer)) > 0) {
        if (kstrtoul(buffer, 10, &memory_usage) != 0) {
            printk(KERN_ERR "Error: No se pudo convertir el uso de memoria\n");
        }
    }
    return memory_usage / (1024 * 1024); // Convertir a MiB
}

// Función para obtener el uso de CPU de un contenedor
static unsigned long long get_cpu_usage(const char *container_id) {
    char path[256];
    char buffer[256];
    unsigned long long cpu_usage1 = 0, cpu_usage2 = 0;
    unsigned long long elapsed_time = 1000000; // 1 segundo en microsegundos

    // Primera lectura de usage_usec
    snprintf(path, sizeof(path), "/sys/fs/cgroup/system.slice/docker-%s.scope/cpu.stat", container_id);
    if (read_file(path, buffer, sizeof(buffer)) > 0) {
        char *line = strstr(buffer, "usage_usec");
        if (line) {
            if (sscanf(line, "usage_usec %llu", &cpu_usage1) != 1) {
                printk(KERN_ERR "Error: No se pudo convertir el uso de CPU\n");
            }
        }
    }

    // Esperar 1 segundo
    msleep(1000);

    // Segunda lectura de usage_usec
    if (read_file(path, buffer, sizeof(buffer)) > 0) {
        char *line = strstr(buffer, "usage_usec");
        if (line) {
            if (sscanf(line, "usage_usec %llu", &cpu_usage2) != 1) {
                printk(KERN_ERR "Error: No se pudo convertir el uso de CPU\n");
            }
        }
    }

    // Calcular la diferencia de uso de CPU
    unsigned long long cpu_delta = cpu_usage2 - cpu_usage1;

    // Calcular el porcentaje de uso de CPU
    unsigned long long cpu_percent = (cpu_delta * 100) / elapsed_time;

    return cpu_percent;
}


// Función para obtener el uso de I/O de un contenedor
static unsigned long long get_io_usage(const char *container_id) {
    char path[256];
    char buffer[256];
    unsigned long long rbytes = 0, wbytes = 0;

    snprintf(path, sizeof(path), "/sys/fs/cgroup/system.slice/docker-%s.scope/io.stat", container_id);
    printk(KERN_INFO "Leyendo I/O desde: %s\n", path);

    if (read_file(path, buffer, sizeof(buffer)) > 0) {
        char *line = buffer;
        while (*line != '\0') {
            // Buscar "rbytes=" en la línea actual
            char *rbytes_pos = strstr(line, "rbytes=");
            if (rbytes_pos) {
                rbytes_pos += strlen("rbytes=");
                unsigned long long current_rbytes;
                if (sscanf(rbytes_pos, "%llu", &current_rbytes) == 1) {
                    rbytes += current_rbytes;
                } else {
                    printk(KERN_ERR "Error: No se pudo convertir rbytes\n");
                }
            }

            // Buscar "wbytes=" en la línea actual
            char *wbytes_pos = strstr(line, "wbytes=");
            if (wbytes_pos) {
                wbytes_pos += strlen("wbytes=");
                unsigned long long current_wbytes;
                if (sscanf(wbytes_pos, "%llu", &current_wbytes) == 1) {
                    wbytes += current_wbytes;
                } else {
                    printk(KERN_ERR "Error: No se pudo convertir wbytes\n");
                }
            }

            // Avanzar a la siguiente línea
            line = strchr(line, '\n');
            if (line) {
                line++;
            } else {
                break;
            }
        }
    }

    // Sumar rbytes y wbytes para obtener el total de I/O
    return (rbytes + wbytes) / (1024 * 1024); // Convertir a MiB
}


// Función para obtener información de los contenedores
static void get_containers_info(struct seq_file *m) {
    struct task_struct *task;
    struct container_info *containers;
    int container_count = 0;

    // Asignar memoria dinámica para los contenedores
    containers = kmalloc_array(MAX_CONTAINERS, sizeof(struct container_info), GFP_KERNEL);
    if (!containers) {
        printk(KERN_ERR "Error: No se pudo asignar memoria para los contenedores\n");
        return;
    }

    // Iterar sobre todos los procesos
    for_each_process(task) {
        // Obtener el ID del contenedor
        const char *container_id = get_container_id(task);
        if (container_id && strcmp(container_id, "docker.service") != 0) {
            // Verificar si el ID ya está en la lista
            int is_duplicate = 0;
            for (int i = 0; i < container_count; i++) {
                if (strncmp(containers[i].id, container_id, CONTAINER_ID_LENGTH) == 0) {
                    is_duplicate = 1;
                    break;
                }
            }

            // Si no es un duplicado, agregarlo a la lista
            if (!is_duplicate && container_count < MAX_CONTAINERS) {
                struct container_info *container = &containers[container_count];
                strncpy(container->id, container_id, CONTAINER_ID_LENGTH);
                container->id[CONTAINER_ID_LENGTH] = '\0'; // Asegurarnos de que esté terminado en NULL

                // Obtener el nombre del contenedor (usamos el ID como nombre por ahora)
                snprintf(container->name, sizeof(container->name), "container_%s", container->id);

                // Obtener el PID del proceso principal del contenedor
                container->pid = task->pid;

                // Obtener las métricas de uso de recursos
                unsigned long memory_usage = get_memory_usage(container_id);
                unsigned long long cpu_usage = get_cpu_usage(container_id);
                unsigned long long io_usage = get_io_usage(container_id);

                // Formatear las métricas
                snprintf(container->memory_usage, sizeof(container->memory_usage), "%lu MiB", memory_usage);
                snprintf(container->cpu_usage, sizeof(container->cpu_usage), "%llu%%", cpu_usage);
                snprintf(container->io_usage, sizeof(container->io_usage), "%llu MiB", io_usage);

                container_count++;
            }
        }
    }

    // Mostrar la información de los contenedores en el JSON
    seq_printf(m, "\t\"Containers\": [\n");
    for (int i = 0; i < container_count; i++) {
        seq_printf(m, "\t\t{\n");
        seq_printf(m, "\t\t\t\"id\": \"%s\",\n", containers[i].id);
        seq_printf(m, "\t\t\t\"name\": \"%s\",\n", containers[i].name);
        seq_printf(m, "\t\t\t\"pid\": %d,\n", containers[i].pid);
        seq_printf(m, "\t\t\t\"memory_usage\": \"%s\",\n", containers[i].memory_usage);
        seq_printf(m, "\t\t\t\"cpu_usage\": \"%s\",\n", containers[i].cpu_usage);
        seq_printf(m, "\t\t\t\"io_usage\": \"%s\"\n", containers[i].io_usage);
        seq_printf(m, "\t\t}%s\n", (i < container_count - 1) ? "," : "");
    }
    seq_printf(m, "\t]\n");

    // Liberar la memoria asignada
    kfree(containers);
}

// Función para mostrar la información en /proc
static int proc_show(struct seq_file *m, void *v) {
    seq_printf(m, "{\n");
    get_containers_info(m);
    seq_printf(m, "}\n");
    return 0;
}

// Función para abrir el archivo en /proc
static int proc_open(struct inode *inode, struct file *file) {
    return single_open(file, proc_show, NULL);
}

// Estructura de operaciones de archivo
static const struct proc_ops proc_fops = {
    .proc_open = proc_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

// Inicialización del módulo
static int __init sysinfo_init(void) {
    proc_create(FILE_NAME, 0, NULL, &proc_fops);
    printk(KERN_INFO "Módulo sysinfo cargado\n");
    return 0;
}

// Finalización del módulo
static void __exit sysinfo_exit(void) {
    remove_proc_entry(FILE_NAME, NULL);
    printk(KERN_INFO "Módulo sysinfo descargado\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Mario Marroquin");
MODULE_DESCRIPTION("Módulo para obtener información de contenedores Docker");