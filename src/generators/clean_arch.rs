use crate::utils::write_file;
use std::path::Path;

pub fn generate_clean_arch(
    base: &Path,
    db: &str,
    orm: &str,
    jwt: bool,
    swagger: bool,
    winston: bool,
) {
    write_file(
        &base.join("tsconfig.json"),
        r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "test"]
}
"#,
    );

    write_file(
        &base.join("src/domain/entities/User.ts"),
        r#"
export class User {

constructor(
public id:number,
public email:string,
public password:string
){}

}
"#,
    );

    write_file(
        &base.join("src/application/usecases/createUser.ts"),
        r#"
export class CreateUser{

execute(){}

}
"#,
    );

    write_file(
        &base.join("src/infrastructure/controllers/userController.ts"),
        r#"
import { Request, Response } from 'express';

export const getUser = (req: Request, res: Response) => {
    res.json({ ok: true, user: { id: 1, email: 'test@example.com' } });
};

export const createUser = (req: Request, res: Response) => {
    res.json({ ok: true, message: 'User created' });
};
"#,
    );

    let route_content = if swagger {
        r#"
import { Router } from 'express';
import { getUser, createUser } from '../controllers/userController';

const router = Router();

/**
 * @swagger
 * /health:
 *   get:
 *     summary: Health check endpoint
 *     responses:
 *       200:
 *         description: Server is healthy
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 status:
 *                   type: string
 *                   example: ok
 */
router.get('/health', (req, res) => {
    res.status(200).json({ status: 'ok' });
});

/**
 * @swagger
 * /:
 *   get:
 *     summary: Root endpoint
 *     responses:
 *       200:
 *         description: Welcome message
 *         content:
 *           text/plain:
 *             schema:
 *               type: string
 *               example: Express API
 */
router.get('/', (req, res) => {
    res.send('Express API');
});

/**
 * @swagger
 * /ping:
 *   get:
 *     summary: Ping endpoint
 *     responses:
 *       200:
 *         description: Pong response
 *         content:
 *           text/plain:
 *             schema:
 *               type: string
 *               example: pong
 */
router.get('/ping', (req, res) => {
    res.send('pong');
});

/**
 * @swagger
 * /user:
 *   get:
 *     summary: Get user
 *     responses:
 *       200:
 *         description: User data
 *   post:
 *     summary: Create user
 *     responses:
 *       200:
 *         description: User created
 */
router.get('/user', getUser);
router.post('/user', createUser);

export default router;
"#
    } else {
        r#"
import { Router } from 'express';
import { getUser, createUser } from '../controllers/userController';

const router = Router();

router.get('/health', (req, res) => {
    res.status(200).json({ status: 'ok' });
});

router.get('/', (req, res) => {
    res.send('Express API');
});

router.get('/ping', (req, res) => {
    res.send('pong');
});

router.get('/user', getUser);
router.post('/user', createUser);

export default router;
"#
    };

    write_file(
        &base.join("src/infrastructure/routes/route.ts"),
        route_content,
    );

    let index_content = if swagger {
        r#"
import express from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import router from './infrastructure/routes/route';
import swaggerUi from 'swagger-ui-express';
import swaggerJsDoc from 'swagger-jsdoc';

dotenv.config();

const app = express();

const swaggerOptions = {
    definition: {
        openapi: '3.0.0',
        info: {
            title: 'Express API',
            version: '1.0.0',
            description: 'API Documentation',
        },
        servers: [
            {
                url: 'http://localhost:' + (process.env.PORT || 3000),
            },
        ],
    },
    apis: ['./src/infrastructure/routes/*.ts'],
};

const swaggerDocs = swaggerJsDoc(swaggerOptions);

app.use(cors());
app.use(express.json());
app.use('/api-docs', swaggerUi.serve, swaggerUi.setup(swaggerDocs));
app.get('/api-docs.json', (req, res) => {
    res.json(swaggerDocs);
});
app.use(router);

const PORT = process.env.PORT || 3000;

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
    console.log(`Swagger docs available at http://localhost:${PORT}/api-docs`);
});

export default app;
"#
    } else {
        r#"
import express from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import router from './infrastructure/routes/route';

dotenv.config();

const app = express();

app.use(cors());
app.use(express.json());
app.use(router);

const PORT = process.env.PORT || 3000;

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});

export default app;
"#
    };

    write_file(&base.join("src/index.ts"), index_content);

    let mut env_vars = vec!["PORT=3000".to_string(), "NODE_ENV=development".to_string()];

    if orm == "Prisma" || orm == "TypeORM" {
        match db {
            "PostgreSQL" => {
                env_vars.push("DB_HOST=localhost".to_string());
                env_vars.push("DB_PORT=5432".to_string());
                env_vars.push("DB_USER=postgres".to_string());
                env_vars.push("DB_PASSWORD=password".to_string());
                env_vars.push("DB_NAME=api_db".to_string());
            }
            "MySQL" => {
                env_vars.push("DB_HOST=localhost".to_string());
                env_vars.push("DB_PORT=3306".to_string());
                env_vars.push("DB_USER=root".to_string());
                env_vars.push("DB_PASSWORD=password".to_string());
                env_vars.push("DB_NAME=api_db".to_string());
            }
            "SQLite" => {
                env_vars.push("DB_PATH=./database.db".to_string());
            }
            _ => {}
        }
    }

    if jwt {
        env_vars.push("JWT_SECRET=your-super-secret-key".to_string());
        env_vars.push("JWT_EXPIRES_IN=24h".to_string());
    }

    if winston {
        env_vars.push("LOG_LEVEL=info".to_string());
    }

    let env_example = env_vars.join("\n");
    let env_content = env_vars.join("\n");

    write_file(&base.join(".env.example"), &env_example);
    write_file(&base.join(".env"), &env_content);
}
