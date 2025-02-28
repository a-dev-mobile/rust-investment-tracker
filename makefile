
#  создавать теги следующим образом:
# make tag TAG=v1.0.0

# Для development релиза
# make tag-dev TAG=dev-v1.0.0



# Определение окружения (по умолчанию local)
ENV ?= local

# Проверка наличия файла окружения
ifneq (,$(wildcard .env.$(ENV)))
    include .env.$(ENV)
    export
else
    $(error .env.$(ENV) file not found. Available environments: dev, local, prod)
endif

# Включаем BuildKit для улучшения производительности
export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1

# Переменные для использования cargo и docker
CARGO := cargo
GIT := git
DOCKER := docker
DOCKER_COMPOSE := docker compose

# Версия проекта из git
VERSION := $(shell $(GIT) rev-parse --short HEAD)

# Имя образа
IMAGE_NAME := investment_tracker

.PHONY: all init clean test build help docker-* print-env use-env tag tag-dev

# Помощь по командам
help:
	@echo "Доступные команды:"
	@echo "  make init          - Инициализация проекта"
	@echo "  make clean         - Очистка"
	@echo "  make test          - Запуск тестов"
	@echo "  make build         - Сборка проекта"
	@echo "  make check         - Проверка кода"
	@echo "  make format        - Форматирование кода"
	@echo "  make docker-build  - Сборка Docker образа"
	@echo "  make docker-run    - Запуск контейнера"
	@echo "  make docker-clean  - Очистка Docker ресурсов"
	@echo "  make docker-restart- Быстрый перезапуск контейнера"
	@echo "  make tag TAG=x.y.z - Создать и отправить production тег"
	@echo "  make tag-dev TAG=dev-vx.y.z - Создать и отправить dev тег"
	@echo "Окружения:"
	@echo "  make use-env ENV=dev   - Использовать dev окружение"
	@echo "  make use-env ENV=local - Использовать local окружение"
	@echo "  make use-env ENV=prod  - Использовать prod окружение"
tag:
	@if [ -z "$(TAG)" ]; then \
		echo "Ошибка: укажите переменную TAG, например: make tag TAG=v1.0.0"; \
		exit 1; \
	fi
	@if [[ "$(TAG)" != v* ]]; then \
		echo "Ошибка: TAG должен начинаться с 'v' (например v1.0.0)"; \
		exit 1; \
	fi
	@echo "Создание production тега $(TAG)..."
	@git tag -a $(TAG) -m "Release $(TAG)"
	@git push origin $(TAG)
	@echo "Тег $(TAG) успешно создан и отправлен"

# Создание dev-тега и отправка его в удалённый репозиторий
tag-dev:
	@if [ -z "$(TAG)" ]; then \
		echo "Ошибка: укажите переменную TAG, например: make tag-dev TAG=dev-v1.0.0"; \
		exit 1; \
	fi
	@if [[ "$(TAG)" != dev-v* ]]; then \
		echo "Ошибка: TAG должен начинаться с 'dev-v' (например dev-v1.0.0)"; \
		exit 1; \
	fi
	@echo "Создание development тега $(TAG)..."
	@git tag -a $(TAG) -m "Development release $(TAG)"
	@git push origin $(TAG)
	@echo "Тег $(TAG) успешно создан и отправлен"
	@echo "Статус workflow можно посмотреть по пути: https://github.com/a-dev-mobile/rust-investment-tracker/actions"


# Показать текущие переменные окружения
print-env:
	@echo "Current environment: $(ENV)"
	@echo "Environment variables from .env.$(ENV):"
	@cat .env.$(ENV)
	@echo "Build variables:"
	@echo "VERSION=$(VERSION)"
	@echo "IMAGE_NAME=$(IMAGE_NAME)"

# Переключение окружения
use-env:
	@echo "Switching to $(ENV) environment"
	@test -f .env.$(ENV) || (echo "Error: .env.$(ENV) not found" && exit 1)
	@ln -sf .env.$(ENV) .env
	@echo "Successfully switched to $(ENV) environment"
	@make print-env

# Инициализация проекта
init: check-versions get clean clean-deps format fix build-dev

# Получение зависимостей
get:
	$(CARGO) fetch

# Очистка
clean:
	$(CARGO) clean
	find . -type f -name "*.rs" -exec sed -i 's/[[:space:]]*$$//' {} +

# Проверка и форматирование кода
format:
	-$(CARGO) fmt

fix:
	$(CARGO) clippy --fix --allow-dirty
	$(CARGO) fmt

check: format
	$(CARGO) clippy -- -D warnings
	$(CARGO) fmt -- --check

# Тестирование
test:
	$(CARGO) test
	$(CARGO) tarpaulin --ignore-tests

# Сборка
build-dev:
	$(CARGO) build

build-release:
	$(CARGO) build --release

# Docker команды (с учетом окружения)
docker-build:
	$(DOCKER) build -t $(IMAGE_NAME):$(ENV) \
		--build-arg BUILD_VERSION=$(VERSION) \
		--build-arg ENV=$(ENV) \
		--progress=plain .

# Остановка всех контейнеров, использующих порт 5000
docker-stop-port:
	@echo "Stopping containers using port 5000..."
	-@$(DOCKER) ps -q --filter publish=5000 | xargs -r $(DOCKER) stop
	@sleep 2  # Даем время на освобождение порта

# Проверка наличия образа
docker-check-image:
	@if ! $(DOCKER) image inspect $(IMAGE_NAME):$(ENV) >/dev/null 2>&1; then \
		echo "Image $(IMAGE_NAME):$(ENV) not found. Building..."; \
		$(MAKE) docker-build; \
	fi

docker-run: docker-stop-port docker-check-image
	@echo "Starting container for $(ENV) environment..."
	$(DOCKER) run -it --rm \
		--env-file .env.$(ENV) \
		--name investment-tracker-$(ENV) \
		-p 5000:5000 \
		-v $(PWD):/app \
		-v cargo-cache:/usr/local/cargo/registry \
		$(IMAGE_NAME):$(ENV)

docker-stop-all:
	@echo "Stopping all containers for $(IMAGE_NAME)..."
	-@$(DOCKER) ps -q --filter ancestor=$(IMAGE_NAME):$(ENV) | xargs -r $(DOCKER) stop
	@echo "All containers stopped"

docker-clean: docker-stop-all
	-$(DOCKER) rmi $(IMAGE_NAME):$(ENV)
	-$(DOCKER) system prune -f

# Быстрый перезапуск для разработки
docker-rebuild: docker-stop-port
	@echo "Rebuilding $(ENV) environment..."
	$(MAKE) docker-build ENV=$(ENV)

docker-restart: docker-stop-port
	@echo "Restarting container for $(ENV) environment..."
	$(MAKE) docker-build ENV=$(ENV)
	$(MAKE) docker-run ENV=$(ENV)

# Команды для быстрого доступа к разным окружениям
dev: 
	$(MAKE) docker-restart ENV=dev

local:
	$(MAKE) docker-restart ENV=local

prod:
	$(MAKE) docker-restart ENV=prod
upgrade:
	$(CARGO) update

clean-deps:
	$(CARGO) clean
	$(CARGO) fetch

check-versions:
	@echo "Environment: $(ENV)"
	@echo "Rust: $$(rustc --version)"
	@echo "Cargo: $$($(CARGO) --version)"
	@echo "Git: $$($(GIT) --version)"
	@echo "Docker: $$($(DOCKER) --version)"
	@echo "Docker Compose: $$($(DOCKER_COMPOSE) --version)"

# Запуск в режиме наблюдения (watch) для разных окружений
watch-dev: use-env
	@ENV=dev $(MAKE) docker-restart

watch-local: use-env
	@ENV=local $(MAKE) docker-restart

watch-prod: use-env
	@ENV=prod $(MAKE) docker-restart

# По умолчанию используем local окружение
watch: watch-local