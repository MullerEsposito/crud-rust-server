# Documentação do Servidor CRUD Rust/Hyper

## Requisitos do Sistema

### Software Necessário
- Docker Desktop
- Git
- Rust (opcional para desenvolvimento local)

### Verificação do Ambiente
Antes de iniciar, certifique-se que o Docker está instalado e rodando:
```bash
docker --version
docker-compose --version
```

## Estrutura do Projeto
```
.
├── src/
│   ├── main.rs           # Servidor Rust
│   └── index.html        # Frontend
├── Dockerfile            # Configuração Docker
├── docker-compose.yml    # Orquestração de containers
├── .github/
│   └── workflows/        # CI/CD
├── nginx.conf            # Configuração do proxy
└── deploy.sh            # Script de deploy
```

## Configuração do Docker

### 1. Instalação do Docker Desktop
- Windows: https://docs.docker.com/desktop/windows/install/
- Mac: https://docs.docker.com/desktop/mac/install/
- Linux: https://docs.docker.com/engine/install/

### 2. Inicialização do Docker Desktop
```bash
# Windows (PowerShell como Administrador)
Start-Service *docker*

# Linux
sudo systemctl start docker
```

### 3. Verificação do Serviço
```bash
docker info
```

## Build e Deploy

### Build Local
```bash
# Certifique-se que o Docker Desktop está rodando
docker build -t rust-server:latest .

# Verificar a imagem criada
docker images
```

### Execução com Docker Compose
```bash
# Iniciar serviços
docker-compose up -d

# Verificar logs
docker-compose logs -f

# Parar serviços
docker-compose down
```

### Solução de Problemas Comuns

#### Erro de Conexão com Docker
Se encontrar o erro:
```
ERROR: error during connect: Head "http://%2F%2F.%2Fpipe%2FdockerDesktopLinuxEngine/_ping"
```

Soluções:
1. Reinicie o Docker Desktop
```bash
# Windows (PowerShell como Administrador)
Restart-Service *docker*

# Linux
sudo systemctl restart docker
```

2. Verifique o status do Docker
```bash
# Windows
Get-Service *docker*

# Linux
sudo systemctl status docker
```

3. Configure o contexto do Docker
```bash
docker context ls
docker context use default
```

## Endpoints da API

### Frontend
- **URL**: `GET /`
- **Descrição**: Interface web do sistema

### Usuários
- **Criar**: `POST /users`
- **Listar**: `GET /users`
- **Atualizar**: `PUT /users/{id}`
- **Deletar**: `DELETE /users/{id}`

## Monitoramento

### Prometheus
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'rust-hyper-api'
    static_configs:
      - targets: ['localhost:3000']
```

### Nginx (Proxy Reverso)
```nginx
# nginx.conf
server {
    listen 80;
    server_name seu-dominio.com;
    
    location / {
        proxy_pass http://localhost:3000;
    }
}
```

## CI/CD

### GitHub Actions
Configurado para deploy automático em push na branch main:
```yaml
# .github/workflows/deploy.yml
name: Deploy
on:
  push:
    branches: [ main ]
```

### Script de Deploy Manual
```bash
# deploy.sh
#!/bin/bash
git pull
docker-compose up -d --build
```

## Segurança
- Configure HTTPS em produção
- Restrinja portas no firewall
- Use variáveis de ambiente para secrets
- Implemente autenticação

## Manutenção
1. Backup regular dos dados
2. Monitoramento de logs
3. Atualização das dependências
4. Verificação de segurança

## Troubleshooting

### Logs do Container
```bash
docker logs rust-server
```

### Reiniciar Serviços
```bash
docker-compose restart
```

### Verificar Rede
```bash
docker network ls
docker network inspect bridge
```

### Limpar Sistema
```bash
docker system prune -a
```

## Contato e Suporte
- Abra uma issue no GitHub
- Documentação completa: [DOCUMENTATION.md](DOCUMENTATION.md)
- Licença: MIT