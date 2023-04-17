-- Criar Tabela solar_factor
CREATE TABLE public.solar_factor (
	id varchar NOT NULL,
	latitude float8 NOT NULL,
	orientation varchar NOT NULL,
	value int4 NOT NULL,
	CONSTRAINT solar_factor_pk PRIMARY KEY (id)
);

-- Inserir Fator Solar
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
