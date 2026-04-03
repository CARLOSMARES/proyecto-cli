use crate::utils::write_file;
use std::path::Path;

pub fn generate_security(base: &Path) {
    let cors_config = r#"
import cors from 'cors';

const corsOptions = {
    origin: process.env.CORS_ORIGIN || '*',
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'PATCH'],
    allowedHeaders: ['Content-Type', 'Authorization'],
    credentials: true,
};

export const corsMiddleware = cors(corsOptions);
"#;

    let helmet_config = r#"
import helmet from 'helmet';

export const securityHeaders = helmet({
    contentSecurityPolicy: {
        directives: {
            defaultSrc: ["'self'"],
            styleSrc: ["'self'", "'unsafe-inline'"],
            scriptSrc: ["'self'"],
            imgSrc: ["'self'", 'data:', 'https:'],
        },
    },
    crossOriginEmbedderPolicy: false,
});
"#;

    let rate_limit_config = r#"
import rateLimit from 'express-rate-limit';

export const limiter = rateLimit({
    windowMs: 15 * 60 * 1000,
    max: 100,
    message: {
        success: false,
        message: 'Too many requests from this IP, please try again later.',
    },
    standardHeaders: true,
    legacyHeaders: false,
});
"#;

    let validation_config = r#"
import { z } from 'zod';

export const loginSchema = z.object({
    email: z.string().email('Invalid email format'),
    password: z.string().min(6, 'Password must be at least 6 characters'),
});

export const registerSchema = z.object({
    email: z.string().email('Invalid email format'),
    password: z.string().min(6, 'Password must be at least 6 characters'),
    confirmPassword: z.string(),
}).refine((data) => data.password === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword'],
});

export const updateUserSchema = z.object({
    email: z.string().email('Invalid email format').optional(),
    password: z.string().min(6, 'Password must be at least 6 characters').optional(),
});

export const validateBody = <T>(schema: z.ZodSchema<T>) => {
    return (req: Request, res: Response, next: NextFunction) => {
        try {
            schema.parse(req.body);
            next();
        } catch (error) {
            if (error instanceof z.ZodError) {
                return res.status(400).json({
                    success: false,
                    message: 'Validation error',
                    errors: error.errors,
                });
            }
            next(error);
        }
    };
};
"#;

    let middlewares_dir = base.join("src/infrastructure/middleware");
    std::fs::create_dir_all(&middlewares_dir).unwrap();

    write_file(&middlewares_dir.join("cors.ts"), &cors_config);
    write_file(&middlewares_dir.join("helmet.ts"), &helmet_config);
    write_file(&middlewares_dir.join("rateLimit.ts"), &rate_limit_config);
    write_file(&middlewares_dir.join("validation.ts"), &validation_config);
}
