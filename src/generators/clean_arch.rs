use crate::utils::write_file;
use std::path::Path;

pub fn generate_clean_arch(
    base: &Path,
    db: &str,
    orm: &str,
    jwt: bool,
    security: bool,
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

    let domain_entities = r#"
export interface BaseEntity {
    id: number;
    createdAt?: Date;
    updatedAt?: Date;
}

export interface IUser extends BaseEntity {
    email: string;
    password: string;
    name?: string;
}

export interface CreateUserDTO {
    email: string;
    password: string;
    name?: string;
}

export interface UpdateUserDTO {
    email?: string;
    password?: string;
    name?: string;
}

export interface UserResponse {
    id: number;
    email: string;
    name?: string;
}
"#;
    write_file(&base.join("src/domain/entities/User.ts"), domain_entities);

    let domain_repository = r#"
import { IUser, CreateUserDTO, UpdateUserDTO, UserResponse } from '../entities/User';

export interface IUserRepository {
    findById(id: number): Promise<UserResponse | null>;
    findByEmail(email: string): Promise<UserResponse | null>;
    findAll(): Promise<UserResponse[]>;
    create(data: CreateUserDTO): Promise<UserResponse>;
    update(id: number, data: UpdateUserDTO): Promise<UserResponse>;
    delete(id: number): Promise<void>;
}
"#;
    write_file(
        &base.join("src/domain/repositories/IUserRepository.ts"),
        domain_repository,
    );

    let use_case_interface = r#"
import { UserResponse, CreateUserDTO, UpdateUserDTO } from '../entities/User';

export interface IUserUseCase {
    getUserById(id: number): Promise<UserResponse>;
    getAllUsers(): Promise<UserResponse[]>;
    createUser(data: CreateUserDTO): Promise<UserResponse>;
    updateUser(id: number, data: UpdateUserDTO): Promise<UserResponse>;
    deleteUser(id: number): Promise<void>;
}
"#;
    write_file(
        &base.join("src/application/usecases/IUserUseCase.ts"),
        use_case_interface,
    );

    let create_user_use_case = r#"
import { IUserRepository } from '../../domain/repositories/IUserRepository';
import { CreateUserDTO, UserResponse } from '../../domain/entities/User';

export class CreateUserUseCase {
    constructor(private userRepository: IUserRepository) {}

    async execute(data: CreateUserDTO): Promise<UserResponse> {
        const existing = await this.userRepository.findByEmail(data.email);
        if (existing) {
            throw new Error('Email already exists');
        }
        return this.userRepository.create(data);
    }
}
"#;
    write_file(
        &base.join("src/application/usecases/CreateUserUseCase.ts"),
        create_user_use_case,
    );

    let get_user_use_case = r#"
import { IUserRepository } from '../../domain/repositories/IUserRepository';
import { UserResponse } from '../../domain/entities/User';

export class GetUserUseCase {
    constructor(private userRepository: IUserRepository) {}

    async execute(id: number): Promise<UserResponse> {
        const user = await this.userRepository.findById(id);
        if (!user) {
            throw new Error('User not found');
        }
        return user;
    }
}
"#;
    write_file(
        &base.join("src/application/usecases/GetUserUseCase.ts"),
        get_user_use_case,
    );

    let delete_user_use_case = r#"
import { IUserRepository } from '../../domain/repositories/IUserRepository';

export class DeleteUserUseCase {
    constructor(private userRepository: IUserRepository) {}

    async execute(id: number): Promise<void> {
        const user = await this.userRepository.findById(id);
        if (!user) {
            throw new Error('User not found');
        }
        return this.userRepository.delete(id);
    }
}
"#;
    write_file(
        &base.join("src/application/usecases/DeleteUserUseCase.ts"),
        delete_user_use_case,
    );

    let user_repository_impl = r#"
import { IUserRepository } from '../../domain/repositories/IUserRepository';
import { CreateUserDTO, UpdateUserDTO, UserResponse } from '../../domain/entities/User';

export class UserRepository implements IUserRepository {
    private users: UserResponse[] = [];

    async findById(id: number): Promise<UserResponse | null> {
        return this.users.find(u => u.id === id) || null;
    }

    async findByEmail(email: string): Promise<UserResponse | null> {
        return this.users.find(u => u.email === email) || null;
    }

    async findAll(): Promise<UserResponse[]> {
        return this.users;
    }

    async create(data: CreateUserDTO): Promise<UserResponse> {
        const newUser: UserResponse = {
            id: this.users.length + 1,
            email: data.email,
            name: data.name,
        };
        this.users.push(newUser);
        return newUser;
    }

    async update(id: number, data: UpdateUserDTO): Promise<UserResponse> {
        const index = this.users.findIndex(u => u.id === id);
        if (index === -1) throw new Error('User not found');
        
        this.users[index] = { ...this.users[index], ...data };
        return this.users[index];
    }

    async delete(id: number): Promise<void> {
        const index = this.users.findIndex(u => u.id === id);
        if (index === -1) throw new Error('User not found');
        this.users.splice(index, 1);
    }
}
"#;
    write_file(
        &base.join("src/infrastructure/repositories/UserRepository.ts"),
        user_repository_impl,
    );

    write_file(
        &base.join("src/infrastructure/controllers/userController.ts"),
        r#"
import { Request, Response, NextFunction } from 'express';
import { CreateUserUseCase } from '../../application/usecases/CreateUserUseCase';
import { GetUserUseCase } from '../../application/usecases/GetUserUseCase';
import { DeleteUserUseCase } from '../../application/usecases/DeleteUserUseCase';
import { UserRepository } from '../repositories/UserRepository';

const userRepository = new UserRepository();
const createUserUseCase = new CreateUserUseCase(userRepository);
const getUserUseCase = new GetUserUseCase(userRepository);
const deleteUserUseCase = new DeleteUserUseCase(userRepository);

export const getUser = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const { id } = req.params;
        const user = await getUserUseCase.execute(Number(id));
        res.json({ success: true, data: user });
    } catch (error) {
        next(error);
    }
};

export const createUser = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const user = await createUserUseCase.execute(req.body);
        res.status(201).json({ success: true, data: user });
    } catch (error) {
        next(error);
    }
};

export const deleteUser = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const { id } = req.params;
        await deleteUserUseCase.execute(Number(id));
        res.json({ success: true, message: 'User deleted' });
    } catch (error) {
        next(error);
    }
};
"#,
    );

    let error_handler = r#"
import { Request, Response, NextFunction } from 'express';

export class AppError extends Error {
    constructor(
        public statusCode: number,
        public message: string,
        public isOperational = true
    ) {
        super(message);
        Object.setPrototypeOf(this, AppError.prototype);
    }
}

export const errorHandler = (
    err: Error,
    req: Request,
    res: Response,
    next: NextFunction
) => {
    if (err instanceof AppError) {
        return res.status(err.statusCode).json({
            success: false,
            message: err.message,
        });
    }

    console.error('Error:', err);
    return res.status(500).json({
        success: false,
        message: 'Internal server error',
    });
};
"#;
    write_file(
        &base.join("src/infrastructure/middleware/errorHandler.ts"),
        error_handler,
    );

    let route_content = if swagger {
        r#"
import { Router } from 'express';
import { getUser, createUser, deleteUser } from '../controllers/userController';
import { errorHandler } from '../middleware/errorHandler';

const router = Router();

router.get('/health', (req, res) => {
    res.status(200).json({ status: 'ok' });
});

router.get('/', (req, res) => {
    res.send('Express API - SOLID Architecture');
});

router.get('/ping', (req, res) => {
    res.send('pong');
});

router.get('/user/:id', getUser);
router.post('/user', createUser);
router.delete('/user/:id', deleteUser);

router.use(errorHandler);

export default router;
"#
    } else {
        r#"
import { Router } from 'express';
import { getUser, createUser, deleteUser } from '../controllers/userController';
import { errorHandler } from '../middleware/errorHandler';

const router = Router();

router.get('/health', (req, res) => {
    res.status(200).json({ status: 'ok' });
});

router.get('/', (req, res) => {
    res.send('Express API - SOLID Architecture');
});

router.get('/ping', (req, res) => {
    res.send('pong');
});

router.get('/user/:id', getUser);
router.post('/user', createUser);
router.delete('/user/:id', deleteUser);

router.use(errorHandler);

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
            description: 'API Documentation - SOLID Architecture',
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

    if orm == "Prisma" || orm == "TypeORM" || orm == "Drizzle" {
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

    if security {
        env_vars.push("CORS_ORIGIN=*".to_string());
    }

    let env_example = env_vars.join("\n");
    let env_content = env_vars.join("\n");

    write_file(&base.join(".env.example"), &env_example);
    write_file(&base.join(".env"), &env_content);
}
