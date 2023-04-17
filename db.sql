--! Criar Tabela solar_factor
CREATE TABLE public.solar_factor (
	id varchar NOT NULL,
	latitude float8 NOT NULL,
	orientation varchar NOT NULL,
	value int4 NOT NULL,
	CONSTRAINT solar_factor_pk PRIMARY KEY (id)
);

--! Inserir Fator Solar
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('740d99dc-d602-45c7-be16-45a706a15a9b', 0.0, 'S', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('1c00a699-6ac2-4b31-bb1d-97400ad410bf', 0.0, 'SE', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('b3ce7bb2-f8ab-48dd-ad5d-a3c87675c0a4', 0.0, 'E', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('207acdd1-1de1-462c-89cf-25dbd8e80ee9', 0.0, 'NE', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('d45ba8b9-6a48-43a4-96cc-adf05a3a4246', 0.0, 'N', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('a7944654-414e-43ba-9884-4edc3a3d85e7', 0.0, 'NO', 273);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('ebbc2ac6-af5d-45f8-83bb-de854bb9de03', 0.0, 'O', 409);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('3e6c1e4a-955d-4c69-8b58-6838b1759bc5', 0.0, 'SO', 273);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('852bf192-c496-470f-87ae-b32315de746c', 10.0, 'S', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('d97bdeec-5e22-4fad-a4bb-e91d694bae59', 10.0, 'SE', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('f2d0c592-31bc-42be-b4aa-d6cee54e2437', 10.0, 'E', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('52201359-ea69-4696-a194-d7cac87f3b87', 10.0, 'NE', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('333ba3c2-98c1-486d-b3c9-57893e962a6b', 10.0, 'N', 51);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('27c0805c-83ed-4dbd-976c-c6176601e389', 10.0, 'NO', 330);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('6dc9db4d-2b7b-45bb-ab75-ffb3638b7d6a', 10.0, 'O', 409);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('a7cbec10-9d73-49f3-ab9f-c7398ecebcdc', 10.0, 'SO', 217);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('bb88c684-0bae-4bde-aa02-f6573a4eec7d', 20.0, 'S', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('4a81dd5d-42ec-4970-bc98-fb2db8e8b036', 20.0, 'SE', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('92fd714a-c67e-4ac7-aea2-b9968859c747', 20.0, 'E', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('3f54627f-dbf0-43c1-a7b7-4d910f75feed', 20.0, 'NE', 35);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('24160944-ba4a-4c2e-b9d2-1d0046fd86d3', 20.0, 'N', 103);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('11a55756-6011-402f-8bba-2f85cabccf75', 20.0, 'NO', 379);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('4e54841e-ebaa-4685-be72-cede7ea5f214', 20.0, 'O', 404);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('2ff49c8f-7707-4963-bcf7-d6f5fea4fd77', 20.0, 'SO', 160);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('5dbf2666-c3f5-4705-ae3b-e076f0417e26', 30.0, 'S', 32);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('a1c16564-4ed5-43af-ac4e-27045c0e4cbc', 30.0, 'SE', 32);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('8a6f2ef1-541e-45d1-8a93-a9477bbb6ce6', 30.0, 'E', 32);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('871423e2-405d-4a13-ad7a-4cebddad2b1d', 30.0, 'NE', 32);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('37cd0ae5-adc6-4e86-bb7f-ff95cc082b4b', 30.0, 'N', 162);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('22ab42e4-e19e-4cdf-a7a6-373d4cc08b32', 30.0, 'NO', 412);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('57ed3901-7f54-43b0-8f3f-79a29f58f508', 30.0, 'O', 390);
INSERT INTO public.solar_factor
(id, latitude, orientation, value)
VALUES('f83da18e-9e73-4bff-bd69-ee855cd2b4cf', 30.0, 'SO', 108);

--? Criar Tabela equipments
CREATE TABLE public.equipments (
	id varchar NOT NULL,
	description varchar NOT NULL,
	power float8 NOT NULL
);

--? Inserir Equipamentos
INSERT INTO public.equipments (id, description, power) VALUES('7478b6f6-07c9-47e5-b656-efb4e88beaa8', 'Computador', 75.0);
INSERT INTO public.equipments (id, description, power) VALUES('da6adb99-04d4-47e2-adad-19dbb5680681', 'Monitor Pequeno', 55.0);
INSERT INTO public.equipments (id, description, power) VALUES('88a05460-ce2f-4145-8d7f-f9ffcd6f5f0f', 'Monitor Médio', 70.0);
INSERT INTO public.equipments (id, description, power) VALUES('6323b1f3-ac02-48fc-b6e0-245ddfbfb309', 'Monitor Grande', 80.0);
INSERT INTO public.equipments (id, description, power) VALUES('fdc36be3-2bef-4398-8212-31a1bd69b0a5', 'Impressora de mesa,pequena', 215.0);
INSERT INTO public.equipments (id, description, power) VALUES('f466145f-00a4-49d8-a039-8eb1aaca641c', 'Impressora de escritório,pequena', 320.0);
INSERT INTO public.equipments (id, description, power) VALUES('558e0db2-6605-47ef-bbac-05faf7448bee', 'Impressora de escritório,grande', 550.0);
INSERT INTO public.equipments (id, description, power) VALUES('f8ab4950-858c-48cb-9d0d-12a6501a8441', 'Copiadoras Grande', 1100.0);
INSERT INTO public.equipments (id, description, power) VALUES('732e5ff8-0a0e-4a6c-bacb-889766b214e9', 'Maquina de Café', 1660.0);
INSERT INTO public.equipments (id, description, power) VALUES('f564c05b-7ae9-4164-8d32-6cfd58ef5493', 'Bebedouro', 1400.0);
INSERT INTO public.equipments (id, description, power) VALUES('3c6878b5-a1ba-4863-bc92-faf8d0c2f6d2', 'Micro-ondas', 2600.0);
INSERT INTO public.equipments (id, description, power) VALUES('81f8e590-0c0e-4875-bee0-c8ee78a5bd6b', 'Torradeira', 5300.0);
INSERT INTO public.equipments (id, description, power) VALUES('9d4effbe-b5c9-416a-8ec3-741f69563550', 'Geladeira', 170.0);
INSERT INTO public.equipments (id, description, power) VALUES('551d5b62-78fc-4903-a00f-23f7d268963a', 'Aparelho de som', 100.0);
INSERT INTO public.equipments (id, description, power) VALUES('fc07807c-f5e2-47d5-ab2d-5060f0a0d781', 'TV', 180.0);

--* Criar Tabela materials
CREATE TABLE public.materials (
	id varchar NOT NULL,
	description varchar NOT NULL,
	conductivity float8 NOT NULL
);

--* Inserir materiais
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Argamassa Comum', 1.15);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Argamassa de Gesso', 0.7);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Argamassa Celular', 0.4);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Cerâmica', 0.9);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Gesso', 0.5);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Fibro-cimento', 0.65);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Concreto Normal', 1.75);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Concreto Cavernoso', 1.4);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Brita', 0.7);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Areia Seca', 0.3);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Areia Saturada', 1.88);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Areia (10% umidade)', 0.93);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Areia (20% umidade)', 1.33);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Lã de vidro', 0.045);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Lã de rocha', 0.045);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Poliestireno expandido moldado', 0.04);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Poliestireno estrudado', 0.035);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Espuma rígida de poliuretano', 0.03);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Borrachas Sintéticas', 0.4);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Acrílicos', 0.2);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Vidro Comum', 1);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Aço', 55);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Alumínio', 230);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Cobre', 380);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Zinco', 112);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Madeiras massa aparente elevada', 0.29);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Carvalho', 0.23);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Freijó', 0.23);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Pinho', 0.23);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Cedro', 0.23);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Pinus', 0.23);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Aglomerado fibras denso (madeira)', 0.2);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Aglomerado fibras leve (madeira)', 0.058);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Aglomerado partículas (madeira)', 0.17);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Placas prensadas (madeira)', 0.12);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Placas extrudadas (madeira)', 0.16);
INSERT INTO public.materials (id, description, conductivity) VALUES (gen_random_uuid(), 'Compensado (madeira)', 0.15);
