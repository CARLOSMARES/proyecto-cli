# Create Proyect CLI 🚀

**Create Proyect CLI** es una herramienta de línea de comandos diseñada para **acelerar la creación y gestión de proyectos de software** mediante una interfaz interactiva rápida y sencilla.

Originalmente desarrollado en **Node.js**, este CLI fue **reescrito completamente en Rust** para ofrecer:

* ⚡ Inicio instantáneo
* 🚀 Alto rendimiento
* 🧠 Experiencia de CLI interactiva moderna
* 📦 Distribución multiplataforma

Con esta herramienta puedes **crear proyectos, clonar repositorios, instalar dependencias y gestionar estructuras completas de aplicaciones** en segundos.

---

# 📦 Instalación

Puedes instalar el CLI desde **dos ecosistemas diferentes**.

## 1️⃣ Instalación con NPM (recomendado para desarrolladores web)

```bash
npm install -g create-proyect-cli
```

## 2️⃣ Instalación con Cargo (usuarios de Rust)

Si tienes Rust instalado:

```bash
cargo install create-proyect-cli
```

---

# 💻 Uso

Una vez instalado, ejecuta el CLI desde cualquier directorio:

```bash
create-proyect-cli
```

Se abrirá un **menú interactivo** donde podrás seleccionar la acción que deseas realizar.

---

# 🛠️ Funcionalidades

## 🚀 Crear Proyecto

Genera automáticamente estructuras completas de proyectos.

### Frontend y Mobile

* Angular
* React
* Vue (Vite)
* Ionic

### Backend (API Express + TypeScript)

Genera una arquitectura backend moderna con:

* rutas
* controladores
* middlewares
* configuración base escalable

Opciones disponibles durante la generación:

**Bases de datos**

* MySQL
* MongoDB
* SQL Server
* MariaDB
* SQLite

**ORMs**

* Prisma
* TypeORM
* Mongoose

**Documentación**

* Swagger (`swagger-ui-express`)

**Contenedores**

* Docker
* Docker Compose

---

### Otros lenguajes

También puedes crear proyectos base en:

**Python**

* proyecto inicial
* entorno virtual (`venv`) automático

**Rust**

* proyecto inicial usando `cargo`

---

# 📂 Acciones Disponibles

Además del generador de proyectos, el CLI permite realizar tareas comunes de desarrollo.

## 📥 Clonar repositorio

Clona cualquier repositorio Git indicando su URL y carpeta destino.

## 📦 Instalar dependencias

Detecta automáticamente el gestor de paquetes y ejecuta la instalación:

* npm
* pnpm
* bun
* deno

## 🗑️ Eliminar repositorio local

Elimina de forma segura directorios o proyectos locales desde la terminal.

---

# 🎯 Objetivo del proyecto

Este CLI busca convertirse en una herramienta para:

* ⚡ acelerar el bootstrap de proyectos
* 🧰 automatizar configuraciones repetitivas
* 🧠 mejorar la productividad de desarrolladores

---

# 🤝 Contribuir

Las contribuciones son bienvenidas.

Puedes ayudar de varias maneras:

* reportando bugs
* proponiendo nuevas features
* enviando pull requests

Repositorio oficial:

<https://github.com/CARLOSMARES/proyecto-cli>

---

# 👨‍💻 Autor

**Carlos Ignacio Olano Mares**

---

# 📄 Licencia

Este proyecto está licenciado bajo **GPL-3.0**.

Consulta el archivo `LICENSE` para más detalles.
