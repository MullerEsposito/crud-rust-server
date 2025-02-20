# API CRUD com Rust e Hyper

Uma API REST com interface web para gerenciamento de usuários, implementada em Rust usando o framework Hyper.

## 🚀 Funcionalidades

- ✅ Criar usuário
- 📋 Listar usuários
- 🔄 Atualizar usuário
- ❌ Deletar usuário
- 🌐 Interface web integrada

## 🛠️ Tecnologias

- Rust
- Hyper (servidor web)
- Tokio (async runtime)
- Serde (serialização)

## 📦 Dependências

```toml
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.3", features = ["v4"] }
```

## 🚀 Como Executar

1. Clone o repositório:
```bash
git clone https://github.com/seu-usuario/rust-hyper-api
cd rust-hyper-api
```

2. Execute o servidor:
```bash
cargo run
```

3. Acesse:
- Frontend: http://localhost:3000
- API: http://localhost:3000/users

## 📡 Endpoints da API

- `GET /` - Interface web
- `POST /users` - Criar usuário
- `GET /users` - Listar usuários
- `PUT /users/{id}` - Atualizar usuário
- `DELETE /users/{id}` - Deletar usuário

## 💡 Exemplos de Uso

### Criar Usuário
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"id": 1, "name": "João", "email": "joao@email.com", "password": "senha123"}'
```

### Listar Usuários
```bash
curl http://localhost:3000/users
```

## ⚠️ Notas Importantes

- Projeto para fins educacionais
- Dados armazenados em memória
- Senhas em texto plano (não use em produção)

## 📝 Documentação

Para documentação detalhada, consulte [DOCUMENTATION.md](DOCUMENTATION.md)

## 🤝 Contribuindo

1. Fork o projeto
2. Crie sua branch (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes. 