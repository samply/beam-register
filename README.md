# Beam Register
Register new apps (key and secret) in Beam through the Beam proxy.

### Requirements

To use the **beam-register** component, ensure the following prerequisites:

1. The **Beam infrastructure** is configured and operational.
2. A **beam-proxy** is running.
3. A job is active that monitors and checks for changes in the file containing the Beam app keys and secrets. This job will restart the **beam-proxy** if it detects any changes.

#### Example Configuration

You can find an example of how to configure the **beam-proxy** and the job in the `docker-compose.yml` file within the [central-datashield-docker repository on GitLab](https://git.verbis.dkfz.de/teiler/central-datashield-docker).<br>
The repository also includes a working example of the job in the `restart-dso-beam-proxy-service` folder, which checks for changes in the relevant file and restarts the **beam-proxy** when necessary.

### Users environment variables file
Please create an environment file and add the path to `BEAM_FILE_PATH`.

### Minimal environment variables
```plaintext
API_KEY=XXXXX
BEAM_FILE_PATH=users.env
RUST_LOG=info
```

### REST API

The **beam-register** component provides a REST API with the following services:

#### 1. `GET /info`
- **Description**: This endpoint returns information about the current version of the Beam Register.
- **Authentication**: No API key or arguments are required.
- **Response**: Information about the current version of the Beam Register.

#### 2. `POST /beam-app`
- **Description**: This endpoint registers an app in Beam.
- **Request Body**:
  ```json
  {
    "beam_id": "test1",
    "beam_secret": "XXX"
  }
  ```
- **Authentication**: Requires an `Authentication` header with the structure:
  ```
  Authentication: ApiKey YOUR_API_KEY
  ```
- **Response**: Confirmation of the app registration.

#### 3. `DELETE /beam-app`
- **Description**: This endpoint unregisters an app from Beam.
- **Request Body**:
  ```json
  {
    "beam_id": "test1"
  }
  ```
- **Authentication**: Requires an `Authentication` header with the structure:
  ```
  Authentication: ApiKey YOUR_API_KEY
  ```
- **Response**: Confirmation of the app unregistration.

#### API Key Authentication
For `POST /beam-app` and `DELETE /beam-app`, authentication is required. The API key must be included in the request header as follows:
```
Authentication: ApiKey YOUR_API_KEY
```
The API key is defined in the `API_KEY` environment variable.

