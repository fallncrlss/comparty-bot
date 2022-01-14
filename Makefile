compose-up:
	docker-compose -f deployment/docker-compose.yml up --build -d

compose-down:
	docker-compose -f deployment/docker-compose.yml down

compose-up-no-app:
	docker-compose -f deployment/docker-compose.yml up -d postgres
	docker-compose -f deployment/docker-compose.yml up -d redis

compose-up-no-build:
	docker-compose -f deployment/docker-compose.yml up -d
