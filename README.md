# RustyUserCSV

**RustyUserCSV** es una aplicación CLI escrita en Rust que implementa un CRUD (Crear, Leer, Actualizar, Eliminar) de usuarios utilizando un archivo CSV como almacenamiento. Este proyecto emplea la biblioteca `clap` para gestionar los comandos que se ejecutan desde la línea de comandos.

## Características

- **Crear Usuarios:** Añade nuevos usuarios al archivo CSV.
- **Leer Todos los Usuarios:** Muestra una lista completa de los usuarios almacenados.
- **Buscar Usuario:** Encuentra un usuario específico mediante su identificador.
- **Editar Usuarios:** Actualiza los datos de un usuario existente.
- **Eliminar Usuarios:** Borra un usuario del archivo CSV.

## Estructura del CSV

El archivo CSV utilizado por **RustyUserCSV** sigue la siguiente estructura:

```csv
Username;Identifier;First name;Last name
```

Ejemplo:

```csv
jdoe01;1;John;Doe
asmith02;2;Alice;Smith
```

## Requisitos

- **Rust**: Asegúrate de tener Rust instalado. Puedes instalarlo desde [aquí](https://www.rust-lang.org/).

## Instalación

1. Clona este repositorio:

   ```bash
   git clone https://github.com/tu-usuario/RustyUserCSV.git
   cd RustyUserCSV
   ```

2. Compila el proyecto:

   ```bash
   cargo build --release
   ```

3. Ejecuta la aplicación:

   ```bash
   cargo run [COMANDO] [ARGUMENTOS]
   ```

## Uso

A continuación, se describen los comandos disponibles y cómo utilizarlos:

### `all`

Muestra todos los usuarios almacenados en el archivo CSV.

```bash
cargo run all
```

### `find`

Busca y muestra un usuario específico mediante su identificador.

```bash
cargo run find [ID]
```

Ejemplo:

```bash
cargo run find 1
```

### `create`

Crea un nuevo usuario. Debes proporcionar un nombre de usuario, nombre y apellido.

```bash
cargo run create -u [USERNAME] -f [FIRST_NAME] -l [LAST_NAME]
```

Ejemplo:

```bash
cargo run create -u jdoe01 -f John -l Doe
```

### `edit`

Edita los detalles de un usuario existente mediante su identificador. Puedes actualizar uno o más campos.

```bash
cargo run edit [ID] -u [NEW_USERNAME] -f [NEW_FIRST_NAME] -l [NEW_LAST_NAME]
```

Ejemplo:

```bash
cargo run edit 1 -f Jonathan -l Doer
```

### `delete`

Elimina un usuario mediante su identificador.

```bash
cargo run delete [ID]
```

Ejemplo:

```bash
cargo run delete 1
```

## Contribuciones

Las contribuciones son bienvenidas. Si deseas contribuir, por favor sigue estos pasos:

1. Haz un fork del proyecto.
2. Crea una nueva rama con tu característica o corrección de errores: `git checkout -b mi-nueva-caracteristica`.
3. Realiza tus cambios y haz un commit: `git commit -m 'Agrega mi nueva característica'`.
4. Envía tus cambios a la rama principal: `git push origin mi-nueva-caracteristica`.
5. Abre un pull request.


## Contacto

Para preguntas o sugerencias, puedes contactarme a través de [linkedin](mailto:https://www.linkedin.com/in/albert-jurado-manjon/).
```