# Create Proyect CLI 🚀

[![npm version](https://img.shields.io/npm/v/create-proyect-cli.svg)](https://www.npmjs.com/package/create-proyect-cli)
[![Crates.io](https://img.shields.io/crates/v/create-proyect-cli.svg)](https://crates.io/crates/create-proyect-cli)
[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](https://opensource.org/licenses/ISC)

Este CLI te permite realizar diversas acciones relacionadas con repositorios y proyectos, proporcionando una interfaz interactiva para simplificar tareas comunes. Originalmente escrito en Node.js y ahora **reescrito en Rust para un rendimiento nativo y tiempos de inicio instantáneos**.

Puedes clonar repositorios, crear proyectos en Angular, React o Ionic, eliminar repositorios y ejecutar comandos npm en repositorios clonados. Además, incluye soporte para proyectos API Express.

## 📦 Instalación

Gracias a su nuevo motor en Rust, puedes instalar este CLI desde el ecosistema que prefieras:

### Opción 1: Vía NPM (Recomendado para desarrolladores web)

Para instalar y utilizar este CLI mediante npm, asegúrate de tener Node.js instalado en tu sistema.

Ejecuta el siguiente comando para instalar el CLI globalmente:

```bash
npm install -g create-proyect-cli
```

### Opción 2: Vía Cargo (Para usuarios de Rust)

Si tienes Rust instalado en tu sistema, puedes compilar e instalar la herramienta de forma nativa directamente desde `crates.io`:

```bash
cargo install create-proyect-cli
```

## 💻 Uso

Una vez instalado, puedes ejecutar el CLI desde cualquier ubicación en tu terminal usando el siguiente comando:

```bash
create-proyect-cli
```

El CLI te guiará a través de las diferentes acciones disponibles, proporcionando opciones interactivas para personalizar tu experiencia.

## 🛠️ Acciones Disponibles

**1. Clonar Repositorio:**
Clona un repositorio Git proporcionando la URL del repositorio y la ubicación de clonación.

**2. Crear Proyecto:**
Crea un nuevo proyecto en Angular, React o Ionic (y API Express). Selecciona el tipo de proyecto y proporciona un nombre.

**3. Eliminar Repositorio:**
Elimina un repositorio localmente. Ingresa la ubicación del repositorio que deseas eliminar.

**4. Ejecutar npm en el Repositorio Clonado:**
Navega hasta la ubicación del repositorio clonado e instala las dependencias con el comando npm.

## 🤝 Contribuir

Si encuentras errores, tienes sugerencias de mejora o deseas contribuir de alguna manera, no dudes en crear un issue o enviar una solicitud de extracción.

**¡Gracias por usar este CLI!**

**¡Esperamos que sea útil para tus proyectos!**

---

## 📄 Licencia y Autoría

* **Autor:** Carlos Ignacio Olano Mares
* **Licencia:** GPL