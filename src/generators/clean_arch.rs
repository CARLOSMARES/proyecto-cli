use crate::utils::write_file;
use std::path::Path;

pub fn generate_clean_arch(base: &Path) {
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
export const createUser=(req,res)=>{

res.json({ok:true})

}
"#,
    );

    write_file(
        &base.join("src/index.ts"),
        r#"
import express from 'express'

const app=express()

app.listen(3000)
"#,
    );
}
