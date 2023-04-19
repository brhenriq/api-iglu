-- ****DDL****

-- ? Create tables
CREATE TABLE solar_factor (
	id varchar NOT NULL,
	latitude float8 NOT NULL,
	orientation varchar NOT NULL,
	value int4 NOT NULL,
	CONSTRAINT solar_factor_pk PRIMARY KEY (id)
);

CREATE TABLE equipments (
	id varchar NOT NULL,
	description varchar NOT NULL,
	power float8 NOT NULL,
	CONSTRAINT equipments_pk PRIMARY KEY (id)
);

CREATE TABLE materials (
	id varchar NOT NULL,
	description varchar NOT NULL,
	conductivity float8 NOT NULL,
	CONSTRAINT materials_pk PRIMARY KEY (id)
);

CREATE TABLE blocks (
	id varchar NOT NULL,
	material_id varchar NOT NULL,
	height float8 NOT NULL,
	width float8 NOT NULL,
	length float8 NOT NULL,
	CONSTRAINT block_pk PRIMARY KEY (id)
);

ALTER TABLE public.blocks ADD CONSTRAINT blocks_fk FOREIGN KEY (material_id) REFERENCES materials(id);

CREATE TABLE public.roofs_types (
	id int4 NOT NULL,
	description varchar NOT NULL,
	CONSTRAINT roofs_types_pk PRIMARY KEY (id)
);

CREATE TABLE public.roofs (
	id varchar NOT NULL,
	material_id varchar NOT NULL,
	thickness float8 NOT NULL,
	type_id int4 NOT NULL,
	CONSTRAINT roofs_pk PRIMARY KEY (id),
	CONSTRAINT roofs_fk FOREIGN KEY (material_id) REFERENCES public.materials(id),
	CONSTRAINT roofs_types_fk FOREIGN KEY (type_id) REFERENCES public.roofs_types(id)
);

-- ! Insert data
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('740d99dc-d602-45c7-be16-45a706a15a9b', 0.0, 'S', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('1c00a699-6ac2-4b31-bb1d-97400ad410bf', 0.0, 'SE', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('b3ce7bb2-f8ab-48dd-ad5d-a3c87675c0a4', 0.0, 'E', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('207acdd1-1de1-462c-89cf-25dbd8e80ee9', 0.0, 'NE', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('d45ba8b9-6a48-43a4-96cc-adf05a3a4246', 0.0, 'N', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('a7944654-414e-43ba-9884-4edc3a3d85e7', 0.0, 'NO', 273);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('ebbc2ac6-af5d-45f8-83bb-de854bb9de03', 0.0, 'O', 409);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('3e6c1e4a-955d-4c69-8b58-6838b1759bc5', 0.0, 'SO', 273);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('852bf192-c496-470f-87ae-b32315de746c', 10.0, 'S', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('d97bdeec-5e22-4fad-a4bb-e91d694bae59', 10.0, 'SE', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('f2d0c592-31bc-42be-b4aa-d6cee54e2437', 10.0, 'E', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('52201359-ea69-4696-a194-d7cac87f3b87', 10.0, 'NE', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('333ba3c2-98c1-486d-b3c9-57893e962a6b', 10.0, 'N', 51);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('27c0805c-83ed-4dbd-976c-c6176601e389', 10.0, 'NO', 330);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('6dc9db4d-2b7b-45bb-ab75-ffb3638b7d6a', 10.0, 'O', 409);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('a7cbec10-9d73-49f3-ab9f-c7398ecebcdc', 10.0, 'SO', 217);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('bb88c684-0bae-4bde-aa02-f6573a4eec7d', 20.0, 'S', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('4a81dd5d-42ec-4970-bc98-fb2db8e8b036', 20.0, 'SE', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('92fd714a-c67e-4ac7-aea2-b9968859c747', 20.0, 'E', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('3f54627f-dbf0-43c1-a7b7-4d910f75feed', 20.0, 'NE', 35);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('24160944-ba4a-4c2e-b9d2-1d0046fd86d3', 20.0, 'N', 103);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('11a55756-6011-402f-8bba-2f85cabccf75', 20.0, 'NO', 379);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('4e54841e-ebaa-4685-be72-cede7ea5f214', 20.0, 'O', 404);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('2ff49c8f-7707-4963-bcf7-d6f5fea4fd77', 20.0, 'SO', 160);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('5dbf2666-c3f5-4705-ae3b-e076f0417e26', 30.0, 'S', 32);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('a1c16564-4ed5-43af-ac4e-27045c0e4cbc', 30.0, 'SE', 32);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('8a6f2ef1-541e-45d1-8a93-a9477bbb6ce6', 30.0, 'E', 32);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('871423e2-405d-4a13-ad7a-4cebddad2b1d', 30.0, 'NE', 32);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('37cd0ae5-adc6-4e86-bb7f-ff95cc082b4b', 30.0, 'N', 162);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('22ab42e4-e19e-4cdf-a7a6-373d4cc08b32', 30.0, 'NO', 412);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('57ed3901-7f54-43b0-8f3f-79a29f58f508', 30.0, 'O', 390);
INSERT INTO solar_factor (id, latitude, orientation, value) VALUES('f83da18e-9e73-4bff-bd69-ee855cd2b4cf', 30.0, 'SO', 108);


INSERT INTO equipments (id, description, power) VALUES('da6adb99-04d4-47e2-adad-19dbb5680681', 'Monitor Pequeno', 55.0);
INSERT INTO equipments (id, description, power) VALUES('88a05460-ce2f-4145-8d7f-f9ffcd6f5f0f', 'Monitor Médio', 70.0);
INSERT INTO equipments (id, description, power) VALUES('6323b1f3-ac02-48fc-b6e0-245ddfbfb309', 'Monitor Grande', 80.0);
INSERT INTO equipments (id, description, power) VALUES('fdc36be3-2bef-4398-8212-31a1bd69b0a5', 'Impressora de mesa,pequena', 215.0);
INSERT INTO equipments (id, description, power) VALUES('f466145f-00a4-49d8-a039-8eb1aaca641c', 'Impressora de escritório,pequena', 320.0);
INSERT INTO equipments (id, description, power) VALUES('558e0db2-6605-47ef-bbac-05faf7448bee', 'Impressora de escritório,grande', 550.0);
INSERT INTO equipments (id, description, power) VALUES('f8ab4950-858c-48cb-9d0d-12a6501a8441', 'Copiadoras Grande', 1100.0);
INSERT INTO equipments (id, description, power) VALUES('732e5ff8-0a0e-4a6c-bacb-889766b214e9', 'Maquina de Café', 1660.0);
INSERT INTO equipments (id, description, power) VALUES('f564c05b-7ae9-4164-8d32-6cfd58ef5493', 'Bebedouro', 1400.0);
INSERT INTO equipments (id, description, power) VALUES('3c6878b5-a1ba-4863-bc92-faf8d0c2f6d2', 'Micro-ondas', 2600.0);
INSERT INTO equipments (id, description, power) VALUES('81f8e590-0c0e-4875-bee0-c8ee78a5bd6b', 'Torradeira', 5300.0);
INSERT INTO equipments (id, description, power) VALUES('9d4effbe-b5c9-416a-8ec3-741f69563550', 'Geladeira', 170.0);
INSERT INTO equipments (id, description, power) VALUES('551d5b62-78fc-4903-a00f-23f7d268963a', 'Aparelho de som', 100.0);
INSERT INTO equipments (id, description, power) VALUES('fc07807c-f5e2-47d5-ab2d-5060f0a0d781', 'TV', 180.0);
INSERT INTO equipments (id, description, power) VALUES('7478b6f6-07c9-47e5-b656-efb4e88beaa8', 'Computador', 75.0);


INSERT INTO materials (id, description, conductivity) VALUES('cff12d13-b6fa-41b5-b6fd-6515a707c382', 'Argamassa Comum', 1.15);
INSERT INTO materials (id, description, conductivity) VALUES('4db81549-1152-4278-a484-768595352b1b', 'Argamassa de Gesso', 0.7);
INSERT INTO materials (id, description, conductivity) VALUES('fa1b421d-e395-4c88-bb4b-42e70505de20', 'Argamassa Celular', 0.4);
INSERT INTO materials (id, description, conductivity) VALUES('24153d27-bf9c-421e-9f96-e1d12f35584e', 'Cerâmica', 0.9);
INSERT INTO materials (id, description, conductivity) VALUES('2ef82717-80e2-4cc2-ad01-458a04beb6f2', 'Gesso', 0.5);
INSERT INTO materials (id, description, conductivity) VALUES('67946aee-f8e2-4ee5-96b3-3c086c9cf414', 'Fibro-cimento', 0.65);
INSERT INTO materials (id, description, conductivity) VALUES('23c2b86a-d048-40ed-bbe2-31e107581814', 'Concreto Normal', 1.75);
INSERT INTO materials (id, description, conductivity) VALUES('8853951c-128f-4558-900d-7ab0febf3405', 'Concreto Cavernoso', 1.4);
INSERT INTO materials (id, description, conductivity) VALUES('0f3a7db5-03e9-4e9f-af97-721233c6f545', 'Brita', 0.7);
INSERT INTO materials (id, description, conductivity) VALUES('af82fdca-54fb-4fa1-ba70-1f67ec5b2336', 'Areia Seca', 0.3);
INSERT INTO materials (id, description, conductivity) VALUES('452a97a9-39fb-45b3-bc59-c8be1561b85c', 'Areia Saturada', 1.88);
INSERT INTO materials (id, description, conductivity) VALUES('c69a4103-fb24-4bf6-b1bd-036a9e0a9730', 'Areia (10% umidade)', 0.93);
INSERT INTO materials (id, description, conductivity) VALUES('89b4b73b-2aec-4bdd-b3e0-40483b8f0e5a', 'Areia (20% umidade)', 1.33);
INSERT INTO materials (id, description, conductivity) VALUES('cd648fdf-3e53-4c07-aec1-87effae785be', 'Lã de vidro', 0.045);
INSERT INTO materials (id, description, conductivity) VALUES('18b721ba-a0a3-47ba-8309-50071aab77d0', 'Lã de rocha', 0.045);
INSERT INTO materials (id, description, conductivity) VALUES('b11c60df-fbd3-4f7b-8419-5b03136df3f8', 'Poliestireno expandido moldado', 0.04);
INSERT INTO materials (id, description, conductivity) VALUES('2bff3535-c862-4de3-adee-adae92b4e2ca', 'Poliestireno estrudado', 0.035);
INSERT INTO materials (id, description, conductivity) VALUES('0536de48-7de2-4feb-a7d4-3e1915743257', 'Espuma rígida de poliuretano', 0.03);
INSERT INTO materials (id, description, conductivity) VALUES('55b0caf2-5f5b-499b-8c21-ae55b171420a', 'Borrachas Sinteticas', 0.4);
INSERT INTO materials (id, description, conductivity) VALUES('eb1bac24-cc42-4a85-be72-fa082c5e1b45', 'Acrilicos', 0.2);
INSERT INTO materials (id, description, conductivity) VALUES('f7aa6c55-6fc7-41a6-9b7c-462d813cdbbf', 'Vidro Comum', 1.0);
INSERT INTO materials (id, description, conductivity) VALUES('534ba3ca-290f-4c8e-a4a5-ae7f3c08dcb0', 'Aço', 55.0);
INSERT INTO materials (id, description, conductivity) VALUES('c929eb77-e623-483e-994e-67a065d07ea2', 'Aluminio', 230.0);
INSERT INTO materials (id, description, conductivity) VALUES('b30e3ea6-0414-4ecb-ba6a-6bd94f268977', 'Cobre', 380.0);
INSERT INTO materials (id, description, conductivity) VALUES('9d52c6d4-cbed-4075-8385-9498dad29935', 'Zinco', 112.0);
INSERT INTO materials (id, description, conductivity) VALUES('883ecc41-7f5e-40d0-abe7-e7dbcb1cb677', 'Madeiras massa aparente elevada', 0.29);
INSERT INTO materials (id, description, conductivity) VALUES('ece1d153-18c0-419e-803b-20b6b546d070', 'Carvalho', 0.23);
INSERT INTO materials (id, description, conductivity) VALUES('f69e45d9-56f5-49a2-ab4d-72440e4e0163', 'Freijó', 0.23);
INSERT INTO materials (id, description, conductivity) VALUES('7acc3259-a472-43b8-8683-e350d4f45023', 'Pinho', 0.23);
INSERT INTO materials (id, description, conductivity) VALUES('5c168104-9113-4994-b4f6-2e929e263ae2', 'Cedro', 0.23);
INSERT INTO materials (id, description, conductivity) VALUES('7c66f6c5-1abc-4cd8-a4d4-48ff6bb50b55', 'Pinus', 0.23);
INSERT INTO materials (id, description, conductivity) VALUES('3fc877f3-0a7f-4405-802d-e425f5e666aa', 'Aglomerado fibras denso (madeira)', 0.2);
INSERT INTO materials (id, description, conductivity) VALUES('d03cce74-8704-4f87-9ba9-e24637456a10', 'Aglomerado fibras leve (madeira)', 0.058);
INSERT INTO materials (id, description, conductivity) VALUES('2a6abdc7-1ada-4ea4-892a-eec8300261f5', 'Aglomerado particulas (madeira)', 0.17);
INSERT INTO materials (id, description, conductivity) VALUES('dbe55740-322f-4190-b342-e5d7d56e0e3b', 'Placas prensadas (madeira)', 0.12);
INSERT INTO materials (id, description, conductivity) VALUES('01e5b845-61ff-47b0-b839-415f829c9f6e', 'Placas extrudadas (madeira)', 0.16);
INSERT INTO materials (id, description, conductivity) VALUES('a1422f44-71b3-49ce-b2c0-cab7b225c479', 'Compensado (madeira)', 0.15);


INSERT INTO blocks (id, material_id, height, width, length) VALUES('eba7beb3-b115-4dcb-99d9-8739b889f64f', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.07, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('ef964d8f-5def-441d-973f-87dee476982f', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.09, 0.29);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('3acebcf3-3f02-47df-8fa9-70c251e12d23', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.09, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('d2cb1487-b02c-4522-ae84-9c17d3af94b5', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.115, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('2c6b2452-7182-49e3-be98-92d2bc8eef43', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.14, 0.29);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('5a7f636e-af86-49e6-a9a6-2d9d177adc78', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.14, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('0ffe14bb-009e-463e-a4fc-837419b87943', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.19, 0.19, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('3c833c0e-3371-4a51-a6d5-58fd76b6cc43', '24153d27-bf9c-421e-9f96-e1d12f35584e', 0.24, 0.24, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('99b4d822-094a-4c35-86a6-03981a8df48f', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.07, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('0c1ccb05-8770-41cf-82d7-77dd3cc49f79', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.09, 0.29);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('337bd09e-3d00-4804-958b-fe66c10d8549', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.09, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('bb462cde-cdd4-4c42-9cdf-2f607fbd6a64', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.115, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('b7e5c616-6ec4-478a-95d1-c90361709a02', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.14, 0.29);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('457703cb-3bcb-448a-b42c-a1539fb3d015', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.14, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('54d7bbee-488a-4da0-9021-56139b2c79df', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.19, 0.19, 0.39);
INSERT INTO blocks (id, material_id, height, width, length) VALUES('9059db52-7892-4bfc-9cae-bc4e3e508e5a', '23c2b86a-d048-40ed-bbe2-31e107581814', 0.24, 0.24, 0.39);

INSERT INTO roofs_types (id, description) VALUES(1, 'lining');
INSERT INTO roofs_types (id, description) VALUES(2, 'tile');
