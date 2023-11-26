#!/usr/bin/env node

import { exec } from 'child_process';
import inquirer from 'inquirer';
import ora from 'ora';
import figlet from 'figlet';
import fs from 'fs'
import path from 'path';

// Función para clonar un repositorio
function cloneRepository(repoUrl, cloneLocation) {
  const cloneCommand = `git clone ${repoUrl} ${cloneLocation}`;
  const cloneSpinner = ora('Clonando repositorio...').start();

  exec(cloneCommand, (error, stdout, stderr) => {
    if (error) {
      cloneSpinner.fail(`Error al clonar el repositorio: ${stderr}`);
    } else {
      cloneSpinner.succeed(`Repositorio clonado exitosamente en: ${cloneLocation}`);
    }
  });
}

//Funcion para instalar dependencias
function installdependencias(location) {
  const install = 'npm install';
  const installSpinner = ora('Instalando Dependencias..').start();
  exec(`cd ${location}`, (error, stdout, stderr) => {
    if (error) {
      installSpinner.fail(`Error al ingresar a la carpeta: ${stderr}`)
    } else {
      exec(install, (error, stdout, stderr) => {
        if (error) {
          installSpinner.fail(`Error al Instalar las dependecias: ${stderr}`);
        } else {
          installSpinner.succeed('Dependencias Instaladas Exitosamente');
        }
      });
    }
  });
}

//Funcion para eliminar repo local
function deleterepo(location) {
  const deleteSpinner = ora('Eliminando Repositorio Local..').start();

  if (fs.existsSync(location)) {
    fs.readdirSync(location).forEach((archivo) => {
      const archivoPath = path.join(location, archivo);

      if (fs.lstatSync(archivoPath).isDirectory()) {
        // Llamamos recursivamente a la función para eliminar el subdirectorio
        deleterepo(archivoPath);
      } else {
        // Si es un archivo, lo eliminamos
        fs.unlinkSync(archivoPath);
      }
    });

    // Finalmente, eliminamos el directorio
    fs.rmdir(location, (error) => {
      if (error) {
        deleteSpinner.fail(`Error al Eliminar el repositorio: ${error}`);
      } else {
        deleteSpinner.succeed('Directorio eliminado correctamente.');
      }
    });
  } else {
    deleteSpinner.fail('El directorio no existe.');
  }
}

// Función para crear un proyecto en Angular, React o Ionic
function createProject(projectType, projectName) {
  let createCommand = '';
  let createSpinnerText = '';

  switch (projectType) {
    case 'Angular':
      createCommand = `npx ng new ${projectName}`;
      createSpinnerText = 'Creando proyecto Angular...';
      break;
    case 'React':
      createCommand = `npx create-react-app ${projectName}`;
      createSpinnerText = 'Creando proyecto React...';
      break;
    case 'Ionic':
      createCommand = `npx ionic start ${projectName} blank`;
      createSpinnerText = 'Creando proyecto Ionic...';
      break;
    case 'api-express':
      createCommand = `git clone https://github.com/CARLOSMARES/api-express.git ${projectName}`;
      createSpinnerText = 'Clonando API Express...';
      break;
    default:
      console.error('Tipo de proyecto no reconocido.');
      return;
  }

  const createSpinner = ora(createSpinnerText).start();

  exec(createCommand, (error, stdout, stderr) => {
    if (error) {
      createSpinner.fail(`Error al crear el proyecto: ${stderr}`);
    } else {
      createSpinner.succeed(`Proyecto ${projectName} creado exitosamente.`);
    }
  });
}

function createAPIExpress(proyectoname) {
  projectType = "api-express";
  createProject(projectType, proyectoname);
}

// Presentación del texto con Figlet
figlet('Create Project CLI', (err, data) => {
  if (err) {
    console.error('Error al generar el texto de presentación.');
    return;
  }
  console.log(data);

  const questions = [
    {
      type: 'list',
      name: 'action',
      message: '¿Qué acción desea realizar?',
      choices: ['Clonar repositorio', 'Crear proyecto', 'Instalar Dependencias', 'Eliminar Repositorio Local'],
    },
    {
      type: 'input',
      name: 'proyectoname',
      message: 'Ingrese el nombre del proyecto:',
      when: (answers) => answers.action === 'api-rest',
    },
    {
      type: 'input',
      name: 'repoUrl',
      message: 'Ingrese la URL del repositorio:',
      when: (answers) => answers.action === 'Clonar repositorio',
    },
    {
      type: 'input',
      name: 'location',
      message: 'Ingrese la URL del repositorio local:',
      when: (answers) => answers.action === 'Eliminar Repositorio Local',
      default: './',
    },
    {
      type: 'input',
      name: 'cloneLocation',
      message: 'Ingrese la ubicación donde desea clonar el repositorio:',
      when: (answers) => answers.action === 'Clonar repositorio',
      default: './',
    },
    {
      type: 'input',
      name: 'location',
      message: 'Ingrese la ubicación donde desea instalar las dependencias:',
      when: (answers) => answers.action === 'Instalar Dependencias',
      default: './',
    },
    {
      type: 'list',
      name: 'projectType',
      message: 'Seleccione el tipo de proyecto:',
      choices: ['Angular', 'React', 'Ionic', `api-express`],
      when: (answers) => answers.action === 'Crear proyecto',
    },
    {
      type: 'input',
      name: 'projectName',
      message: 'Ingrese el nombre del proyecto:',
      when: (answers) => answers.action === 'Crear proyecto',
    },
  ];

  inquirer.prompt(questions).then((answers) => {
    if (answers.action === 'Clonar repositorio') {
      cloneRepository(answers.repoUrl, answers.cloneLocation);
    } else if (answers.action === 'Crear proyecto') {
      createProject(answers.projectType, answers.projectName);
    } else if (answers.action === 'Instalar Dependencias') {
      installdependencias(answers.location);
    } else if (answers.action === 'Eliminar Repositorio Local') {
      deleterepo(answers.location);
    } else if (answers.action === 'api-express') {
      createAPIExpress(answers.proyectoname);
    }
  });
});
