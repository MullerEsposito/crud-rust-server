#!/bin/bash

# Atualizar código
git pull origin main

# Reconstruir e reiniciar containers
docker-compose down
docker-compose build
docker-compose up -d

# Verificar status
docker-compose ps 