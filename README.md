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

El CLI soporta dos modos de operación:

## Modo Interactivo (por defecto)

```bash
create-proyect-cli
```

Se abrirá un **menú interactivo** donde podrás seleccionar la acción que deseas realizar.

## Modo Comandos

```bash
# Crear proyecto React
create-proyect-cli create --template=react --name=my-app

# Crear proyecto Vue
create-proyect-cli generate --template=vue --name=my-vue-app

# Crear API Express con Drizzle
create-proyect-cli new --template=api --name=my-api --orm=drizzle --database=postgresql --jwt --security --swagger

# Ver ayuda
create-proyect-cli --help
create-proyect-cli create --help
```

---

# 🛠️ Comandos y Banderas

## Subcomandos

| Comando | Descripción |
|---------|-------------|
| `create` | Crear un nuevo proyecto |
| `generate` | Generar un nuevo proyecto (alias de create) |
| `new` | Crear un nuevo proyecto (alias de create) |
| `interactive` | Iniciar el CLI en modo interactivo |

## Banderas Disponibles

| Bandera | Descripción | Valores |
|--------|------------|---------|
| `--template` | Tipo de proyecto | `react`, `vue`, `angular`, `ionic`, `api`, `rust`, `python` |
| `--name` | Nombre del proyecto | string |
| `--orm` | ORM a usar (API) | `drizzle`, `prisma`, `typeorm` |
| `--database` | Base de datos | `mysql`, `postgresql`, `sqlite` |
| `--jwt` | Incluir autenticación JWT | flag |
| `--security` | Incluir seguridad (helmet, rate-limit, zod) | flag |
| `--swagger` | Incluir Swagger | flag |
| `--jest` | Incluir Jest para testing | flag |
| `--winston` | Incluir Winston para logs | flag |
| `--docker` | Incluir Docker | flag |
| `--docker-compose` | Incluir Docker Compose con DB | flag |
| `--git` | Inicializar repositorio Git | flag |
| `--install` | Instalar dependencias automáticamente | flag |

---

# 🛠️ Funcionalidades

## 🚀 Crear Proyecto

Genera automáticamente estructuras completas de proyectos.

### Frontend y Mobile

* **React** - Con Vite (`npm create vite`)
* **Vue** - Con Vite (`npm create vite`)
* **Angular**
* **Ionic**

### Backend (API Express + TypeScript)

Genera una arquitectura **SOLID** moderna con:

```
src/
├── domain/
│   ├── entities/       # Interfaces y DTOs
│   └── repositories/   # Interfaces de repositorio
├── application/
│   └── usecases/      # Casos de uso
└── infrastructure/
    ├── controllers/    # Controladores
    ├── middleware/    # Middlewares (auth, error handling)
    ├── repositories/  # Implementaciones
    └── routes/       # Rutas
```

**Opciones de ORM:**

* **Drizzle** - ORM moderno y liviano
* **Prisma** - ORM robusto con migración
* **TypeORM** - ORM con patrón ActiveRecord

**Seguridad integrada:**

* **Helmet** - Headers de seguridad
* **express-rate-limit** - Limitación de peticiones
* **Zod** - Validación de datos
* **CORS** - Configuración de CORS
* **JWT** - Autenticación con JSON Web Tokens
* **bcryptjs** - Hash de contraseñas

**Documentación:**

* **Swagger/OpenAPI** - Documentación automática (`swagger-ui-express`)

**Logging:**

* **Winston** - Sistema de logs configurable

**Testing:**

* **Jest** - Framework de testing

**Contenedores:**

* **Docker** - Imagen optimizada
* **Docker Compose** - Con base de datos

### Python

* Proyecto base con `venv` automático

### Rust

* Proyecto base usando `cargo`

---

# 📂 Acciones Disponibles

Además del generador de proyectos, el CLI permite realizar tareas comunes de desarrollo.

## 📥 Clonar repositorio

Clona cualquier repositorio Git indicando su URL y carpeta destino.

## 📦 Instalar dependencias

Detecta automáticamente el gestor de paquetes:

* npm
* pnpm
* bun
* deno

## 🗑️ Eliminar repositorio local

Elimina de forma segura directorios o proyectos locales.

---

# 🎯 Ejemplos

```bash
# Crear proyecto React con todas las opciones
create-proyect-cli create --template=react --name=my-app --install

# Crear API Express con Drizzle y todas las features
create-proyect-cli create --template=api --name=my-api \
  --orm=drizzle \
  --database=postgresql \
  --jwt \
  --security \
  --swagger \
  --jest \
  --winston \
  --docker \
  --docker-compose \
  --git \
  --install

# Crear proyecto Vue
create-proyect-cli generate --template=vue --name=my-vue-app --install
```

---

# 🎯 Objetivo del proyecto

Este CLI busca convertirse en una herramienta para:

* ⚡ acelerar el bootstrap de proyectos
* 🧰 automatizar configuraciones repetitivas
* 🧠 mejorar la productividad de desarrolladores

---

# 🤝 Contribuir

Las contribuciones son bienvenidas. Puedes ayudar de varias maneras:

* reportando bugs
* proponiendo nuevas features
* enviando pull requests

**Repositorio oficial:**

https://github.com/CARLOSMARES/proyecto-cli

---

# 👨‍💻 Autor

**Carlos Ignacio Olano Mares**

---

# 📄 Licencia

Este proyecto está licenciado bajo **GPL-3.0**.

Consulta el archivo `LICENSE` para más detalles.