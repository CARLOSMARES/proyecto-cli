# Create Proyect CLI 🚀

[![npm version](https://img.shields.io/npm/v/create-proyect-cli.svg)](https://www.npmjs.com/package/create-proyect-cli)
[![Crates.io](https://img.shields.io/crates/v/create-proyect-cli.svg)](https://crates.io/crates/create-proyect-cli)
[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](https://opensource.org/licenses/ISC)

Este CLI te permite realizar diversas acciones relacionadas con repositorios y proyectos, proporcionando una interfaz interactiva ultrarrápida para simplificar tareas comunes. Originalmente escrito en Node.js y ahora **reescrito en Rust para un rendimiento nativo y tiempos de inicio instantáneos**.

Con esta herramienta puedes clonar repositorios, inicializar proyectos en múltiples lenguajes y frameworks, configurar bases de datos, contenedores Docker y gestionar dependencias de manera fluida.

## 📦 Instalación

Gracias a su nuevo motor en Rust, puedes instalar este CLI desde el ecosistema que prefieras:

### Opción 1: Vía NPM (Recomendado para desarrolladores web)

Si ya utilizas el ecosistema de JavaScript/Node.js, instala el binario globalmente a través de npm:

```bash
npm install -g create-proyect-cli
```

### Opción 2: Vía Cargo (Para usuarios de Rust)

Si tienes Rust instalado en tu sistema, puedes compilar e instalar la herramienta directamente desde `crates.io`:

```bash
cargo install create-proyect-cli
```

## 💻 Uso

Una vez instalado, abre tu terminal en cualquier ubicación y ejecuta:

```bash
create-proyect-cli
```

El menú interactivo te guiará a través de las diferentes acciones disponibles, sin necesidad de recordar comandos o banderas complejas.

## 🛠️ Acciones Disponibles

### 1. Crear Proyecto (Generador Inteligente)
Genera la estructura base para nuevos proyectos. El CLI soporta múltiples ecosistemas y te guiará con configuraciones interactivas:

* **Frontend & Móvil:**
  * Angular
  * React
  * Vue (Vite)
  * Ionic
* **Backend Avanzado (API Express - TypeScript):**
  * Generación de estructura escalable (rutas, controladores, middlewares).
  * **Bases de datos:** Elección interactiva entre MySQL, MongoDB, SQL Server, MariaDB o SQLite.
  * **ORMs:** Configuración automática de Prisma, TypeORM o Mongoose.
  * **Documentación:** Instalación opcional de Swagger (`swagger-ui-express`).
  * **Docker:** Generación automática de `Dockerfile` y `docker-compose.yml` vinculando tu API con la base de datos seleccionada.
* **Otros Lenguajes:**
  * **Python:** Crea un proyecto base e inicializa automáticamente su entorno virtual (`venv`).
  * **Rust:** Inicializa un proyecto nativo con Cargo.

### 2. Clonar Repositorio
Clona un repositorio Git proporcionando la URL y la ubicación destino, mostrando el progreso en tiempo real.

### 3. Instalar Dependencias
Navega automáticamente a la ubicación de tu proyecto y ejecuta la instalación de dependencias (`npm install`) en segundo plano.

### 4. Eliminar Repositorio Local
Borra de forma segura y recursiva un directorio o repositorio local especificando su ruta.

## 🤝 Contribuir

Si encuentras errores, tienes sugerencias de mejora o deseas agregar soporte para nuevos lenguajes, no dudes en crear un *issue* o enviar un *pull request* en el repositorio oficial.

**¡Gracias por usar este CLI! Esperamos que acelere la creación de todos tus proyectos.**

---

## 📄 Licencia y Autoría

* **Autor:** Carlos Ignacio Olano Mares
* **Licencia:** GPL