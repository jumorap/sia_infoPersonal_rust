SHELL := /bin/bash

build:
	docker build --progress=plain -t jumorap/sia_info_personal_ms .

run:
	docker run -p 8000:8000 --name sia_info_personal_ms jumorap/sia_info_personal_ms

runAll:
	docker run -p 8000:8000 --name sia_info_personal_ms jumorap/sia_info_personal_ms & docker run -p 8888:3001 --name sia_2e_infoacademica_ms paulinoacuna/2e_infoacademica_ms & docker run -p 6000:3000 --name sia_session_ms ycuervob/sia_session & docker run -p 49160:4000 -d jleonro/architecture2022

runServer:
	docker run -d -t -i -p 3306:3306 --name sia_asignaturas_db jorodriguezal/sia_asignaturas_db
	docker run --name db_client -d --link sia_asignaturas_db:db -p 8081:80 phpmyadmin

runFinal:
	docker run -p 4000:4000 -e DB_HOST=host.docker.internal -e DB_PORT=3306 -e DB_USER=admin -e DB_PASSWORD=2022 -e DB_NAME=sia_asignaturas_db -e URL=0.0.0.0:4000 jorodriguezal/sia_inscripciones_ms
