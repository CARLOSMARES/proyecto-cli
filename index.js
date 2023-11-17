import { exec } from 'child_process';
import inquirer from 'inquirer';
import ora from 'ora';
import figlet from 'figlet';

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

// Presentación del texto con Figlet
figlet('Project CLI', (err, data) => {
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
      choices: ['Clonar repositorio', 'Crear proyecto'],
    },
    {
      type: 'input',
      name: 'repoUrl',
      message: 'Ingrese la URL del repositorio:',
      when: (answers) => answers.action === 'Clonar repositorio',
    },
    {
      type: 'input',
      name: 'cloneLocation',
      message: 'Ingrese la ubicación donde desea clonar el repositorio:',
      when: (answers) => answers.action === 'Clonar repositorio',
      default: './',
    },
    {
      type: 'list',
      name: 'projectType',
      message: 'Seleccione el tipo de proyecto:',
      choices: ['Angular', 'React', 'Ionic'],
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
    }
  });
});
