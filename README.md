## Backend for education-platform

### Для запуска потребуется
- Клонировать репозиторий

```bash
git clone https://github.com/cookies-with-tea/education-platform -b backend/development
```
- Скопировать `.env` 
```bash
cp .env.example .env
```
- Заполнить `.env`
- Изменить POSTGRES_HOST = `db`
- Запустить контейнеры
```bash
docker-compose up -d --build
```

### Для локальной разработки
- Изменить POSTGRES_HOST = `localhost`
```bash
docker-compose up db -d --build
```

<hr />

### Подсказки, чтобы не забыть

**fix postgres with sqlx** - add postgres to features
```
cargo add sqlx -F postgres
```

**add new migration**
```
sqlx migrate add -r <name>
```

**hot reload**
```
cargo watch -x run
```
