# Переменные для использования cargo
CARGO := cargo
GIT := git

# Версия проекта из git
VERSION := $(shell $(GIT) rev-parse --short HEAD)

.PHONY: all init clean test build help

# Помощь по командам
help:
	@echo "Доступные команды:"
	@echo "  make init          - Инициализация проекта"
	@echo "  make clean         - Очистка"
	@echo "  make test          - Запуск тестов"
	@echo "  make build         - Сборка проекта"
	@echo "  make check         - Проверка кода"
	@echo "  make format        - Форматирование кода"

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

# Запуск для разных окружений
run-local: build-dev
	APP_ENV=local $(CARGO) run

run-dev: build-dev
	APP_ENV=dev $(CARGO) run

run-prod: build-release
	APP_ENV=prod $(CARGO) run

# По умолчанию используем local окружение
run: run-local

# Запуск с отслеживанием изменений для разных окружений
watch-local:
	APP_ENV=local $(CARGO) watch -x run

watch-dev:
	APP_ENV=dev $(CARGO) watch -x run

watch-prod:
	APP_ENV=prod $(CARGO) watch -x run

# По умолчанию используем local окружение
watch: watch-local

# Управление зависимостями
upgrade:
	$(CARGO) update

clean-deps:
	$(CARGO) clean
	$(CARGO) fetch

check-versions:
	@echo "Rust: $$(rustc --version)"
	@echo "Cargo: $$($(CARGO) --version)"
	@echo "Git: $$($(GIT) --version)"