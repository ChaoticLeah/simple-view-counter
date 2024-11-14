# Simple View Counter

<center><img alt="Docker Image Size" src="https://img.shields.io/docker/image-size/chaoticleah/simple-view-counter?link=https%3A%2F%2Fhub.docker.com%2Frepository%2Fdocker%2Fchaoticleah%2Fsimple-view-counter%2Fgeneral">
</center>


**Simple View Counter** is a page view counter API, built in Rust, that follows the KISS (Keep It Simple, Stupid) principles. It is a lightweight and easy-to-integrate solution designed to help you count page views efficiently. This self-hosted API can be seamlessly integrated into websites with minimal setup, providing an effective way to track page views.

## Features

- **Customizable Cooldown Period**: Set a cooldown period to limit the frequency of page views counted per IP.
- **Simple Configuration**: Easily configurable via a YAML file.
- **Super fast and lightweight**: Built with Rust for speed and efficiency. 🚀
  
## Installation

### Docker (Recommended)

Make a docker-compose.yml:
```yml
version: '3.8'

services:
    simple_view_counter:
        image: chaoticleah/simple-view-counter:latest
        container_name: view_counter
        volumes:
            - ./app:/app
        ports:
            - "8080:8080"
```

Run `docker compose up` or `docker compose up -d` to keep it running in the background

### Build it yourself

#### Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).

#### Clone the Repository

```bash
git clone https://github.com/ChaoticLeah/simple-view-counter.git
cd simple-view-counter
```

#### Build the Project

Run the following command to build the project:

```bash
cargo build --release
```

#### Run the project

Run the project by running the built file with these arguments:
`view_counter.exe --config /app/config.yaml --db /app/data.db`

### Configuration

The api can be configured with a config.yaml file:
```yaml
# Copy this config file and name it config.yaml. Place it in the same directory as the executable.

allowed_origin: "http://localhost:8080" # Allowed origin for CORS (Make this the page you want to send the requests from)
cooldown: 12 # How many hours to wait before accepting another view from the same IP on the same route. If the server restarts then it will also accept new views
log_level: "off" # error, warn, info, debug, trace, or off (useful for debugging issues)
blacklist_ips: # List of IPs to blacklist (Consider putting your own IP here so you dont count your own views)
  - "your-ip"
```

## Contributing

Feel free to open issues or submit pull requests for any bug fixes, improvements, or features!

## License

This project is licensed under the GNU Affero General Public License. See the [LICENSE](LICENSE) file for more details.
