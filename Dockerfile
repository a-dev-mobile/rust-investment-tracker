# Stage 1: Сборка приложения
FROM rust:1.84.0 AS builder

# Установка рабочей директории внутри контейнера
WORKDIR /app

# Копирование файлов зависимостей
COPY Cargo.toml Cargo.lock ./

# Копирование build.rs
COPY build.rs ./

# Копирование исходного кода и конфигурационных файлов
COPY src ./src
COPY config ./config

# Копирование статических файлов
COPY static ./static

# Установка переменной окружения для сборки
ENV APP_ENV=local

# Сборка проекта в режиме релиза
RUN cargo build --release

# Stage 2: Создание финального образа
FROM debian:bookworm-slim

# Установка необходимых библиотек
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Установка рабочей директории
WORKDIR /usr/local/bin

# Копирование скомпилированного бинарного файла из предыдущего этапа
COPY --from=builder /app/target/release/thread_api .

# Копирование конфигурационных файлов из предыдущего этапа
COPY --from=builder /app/config /usr/local/bin/config

# Копирование статических файлов
COPY --from=builder /app/static /usr/local/bin/static

# Установка переменной окружения для выбора конфигурации устанавливаемой среды из скриптов деплоя
ENV APP_ENV=prod

# Открытие порта 5000
EXPOSE 5000

# Запуск приложения
CMD ["./thread_api"]
