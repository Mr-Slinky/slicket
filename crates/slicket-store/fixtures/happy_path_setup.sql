-- The happy path fixture fills a migrated database with a tenant, the organisations it serves, and
-- the ticket types and ticket statuses its staff set up. The tenant is Slinky IT, and each
-- organisation after it has its own set of people.
--
-- Every row states its own id. Each id column is GENERATED ALWAYS, meaning PostgreSQL accepts a
-- stated id only from an INSERT that says OVERRIDING SYSTEM VALUE.
--
-- Each organisation takes an INSERT for its org row. A second INSERT beneath it adds the
-- organisation's people, giving each one a person_id and the org_id of that organisation. The
-- person_id values count up from 1 in file order.
--
-- Every email in the file is unique, ignoring case. The person_email_idx index on person rejects
-- a second copy of an address.

-- Slinky IT, the tenant. The tenant table stores the org_id of this one org row.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (1, 'Slinky IT');

INSERT
INTO tenant (org_id)
VALUES (1);

-- Slinky IT's 15 staff, all on slicket.com.
INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (1, 1, 'Thandiwe Nkosi', 'thandiwe.nkosi@slicket.com'),
       (2, 1, 'Pieter van der Merwe', 'pieter.vandermerwe@slicket.com'),
       (3, 1, 'Aisha Patel', 'aisha.patel@slicket.com'),
       (4, 1, 'Sipho Dlamini', 'sipho.dlamini@slicket.com'),
       (5, 1, 'Megan O''Connor', 'megan.oconnor@slicket.com'),
       (6, 1, 'Lerato Mokoena', 'lerato.mokoena@slicket.com'),
       (7, 1, 'Johan Botha', 'johan.botha@slicket.com'),
       (8, 1, 'Priya Naidoo', 'priya.naidoo@slicket.com'),
       (9, 1, 'Kagiso Molefe', 'kagiso.molefe@slicket.com'),
       (10, 1, 'Chloe Adams', 'chloe.adams@slicket.com'),
       (11, 1, 'Tshepo Mahlangu', 'tshepo.mahlangu@slicket.com'),
       (12, 1, 'Ruan Steyn', 'ruan.steyn@slicket.com'),
       (13, 1, 'Nomvula Zulu', 'nomvula.zulu@slicket.com'),
       (14, 1, 'Daniel Fourie', 'daniel.fourie@slicket.com'),
       (15, 1, 'Zanele Khumalo', 'zanele.khumalo@slicket.com');

-- Harbourview Dental, a small practice with 12 staff on its own domain.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (2, 'Harbourview Dental');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (16, 2, 'Nadia Hendricks', 'nadia.hendricks@harbourviewdental.co.za'),
       (17, 2, 'Craig Williams', 'craig.williams@harbourviewdental.co.za'),
       (18, 2, 'Fatima Isaacs', 'fatima.isaacs@harbourviewdental.co.za'),
       (19, 2, 'Bongani Ndlovu', 'bongani.ndlovu@harbourviewdental.co.za'),
       (20, 2, 'Liezl Pretorius', 'liezl.pretorius@harbourviewdental.co.za'),
       (21, 2, 'Ayanda Mthembu', 'ayanda.mthembu@harbourviewdental.co.za'),
       (22, 2, 'Shannon Jacobs', 'shannon.jacobs@harbourviewdental.co.za'),
       (23, 2, 'Riaan du Plessis', 'riaan.duplessis@harbourviewdental.co.za'),
       (24, 2, 'Kavitha Govender', 'kavitha.govender@harbourviewdental.co.za'),
       (25, 2, 'Lindiwe Sithole', 'lindiwe.sithole@harbourviewdental.co.za'),
       (26, 2, 'Marco Ferreira', 'marco.ferreira@harbourviewdental.co.za'),
       (27, 2, 'Chloe Adams', 'chloe.adams@harbourviewdental.co.za');

-- Oakridge Primary School, a medium-sized school with 30 staff on personal gmail.com addresses.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (3, 'Oakridge Primary School');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (28, 3, 'Elaine Visser', 'elaine.visser@gmail.com'),
       (29, 3, 'Themba Cele', 'themba.cele@gmail.com'),
       (30, 3, 'Rochelle Petersen', 'rochelle.petersen@gmail.com'),
       (31, 3, 'Sipho Dlamini', 'sipho.dlamini@gmail.com'),
       (32, 3, 'Karen Smit', 'karen.smit@gmail.com'),
       (33, 3, 'Mandla Shabalala', 'mandla.shabalala@gmail.com'),
       (34, 3, 'Natasha Moodley', 'natasha.moodley@gmail.com'),
       (35, 3, 'Gert Swanepoel', 'gert.swanepoel@gmail.com'),
       (36, 3, 'Palesa Mofokeng', 'palesa.mofokeng@gmail.com'),
       (37, 3, 'Brendan Lewis', 'brendan.lewis@gmail.com'),
       (38, 3, 'Nokuthula Dube', 'nokuthula.dube@gmail.com'),
       (39, 3, 'Anita Reddy', 'anita.reddy@gmail.com'),
       (40, 3, 'Willem Joubert', 'willem.joubert@gmail.com'),
       (41, 3, 'Zodwa Ngcobo', 'zodwa.ngcobo@gmail.com'),
       (42, 3, 'Candice Abrahams', 'candice.abrahams@gmail.com'),
       (43, 3, 'Lucky Maluleke', 'lucky.maluleke@gmail.com'),
       (44, 3, 'Marietjie Kruger', 'marietjie.kruger@gmail.com'),
       (45, 3, 'Sibusiso Mkhize', 'sibusiso.mkhize@gmail.com'),
       (46, 3, 'Tamsin Clarke', 'tamsin.clarke@gmail.com'),
       (47, 3, 'Refilwe Motaung', 'refilwe.motaung@gmail.com'),
       (48, 3, 'Deon Oosthuizen', 'deon.oosthuizen@gmail.com'),
       (49, 3, 'Hlengiwe Buthelezi', 'hlengiwe.buthelezi@gmail.com'),
       (50, 3, 'Samantha Davids', 'samantha.davids@gmail.com'),
       (51, 3, 'Vusi Radebe', 'vusi.radebe@gmail.com'),
       (52, 3, 'Charmaine Olivier', 'charmaine.olivier@gmail.com'),
       (53, 3, 'Neo Seabi', 'neo.seabi@gmail.com'),
       (54, 3, 'Ilse Venter', 'ilse.venter@gmail.com'),
       (55, 3, 'Ashwin Pillay', 'ashwin.pillay@gmail.com'),
       (56, 3, 'Busisiwe Khoza', 'busisiwe.khoza@gmail.com'),
       (57, 3, 'Gavin Meyer', 'gavin.meyer@gmail.com');

-- Karoo Freight, a large company with 100 employees on karoofreight.co.za.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (4, 'Karoo Freight');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (58, 4, 'Hendrik Coetzee', 'hendrik.coetzee@karoofreight.co.za'),
       (59, 4, 'Nompumelelo Mabaso', 'nompumelelo.mabaso@karoofreight.co.za'),
       (60, 4, 'Jason Naicker', 'jason.naicker@karoofreight.co.za'),
       (61, 4, 'Carla Bezuidenhout', 'carla.bezuidenhout@karoofreight.co.za'),
       (62, 4, 'Lwazi Hadebe', 'lwazi.hadebe@karoofreight.co.za'),
       (63, 4, 'Stefan Nel', 'stefan.nel@karoofreight.co.za'),
       (64, 4, 'Precious Mashaba', 'precious.mashaba@karoofreight.co.za'),
       (65, 4, 'Ravi Singh', 'ravi.singh@karoofreight.co.za'),
       (66, 4, 'Annelie du Toit', 'annelie.dutoit@karoofreight.co.za'),
       (67, 4, 'Mpho Tau', 'mpho.tau@karoofreight.co.za'),
       (68, 4, 'Grant Robertson', 'grant.robertson@karoofreight.co.za'),
       (69, 4, 'Thuli Mnguni', 'thuli.mnguni@karoofreight.co.za'),
       (70, 4, 'Riyaad Salie', 'riyaad.salie@karoofreight.co.za'),
       (71, 4, 'Jacques Lombard', 'jacques.lombard@karoofreight.co.za'),
       (72, 4, 'Nosipho Gumede', 'nosipho.gumede@karoofreight.co.za'),
       (73, 4, 'Wayne Hartley', 'wayne.hartley@karoofreight.co.za'),
       (74, 4, 'Keabetsoe Letsie', 'keabetsoe.letsie@karoofreight.co.za'),
       (75, 4, 'Francois Rossouw', 'francois.rossouw@karoofreight.co.za'),
       (76, 4, 'Shireen Kader', 'shireen.kader@karoofreight.co.za'),
       (77, 4, 'Mzwandile Ntuli', 'mzwandile.ntuli@karoofreight.co.za'),
       (78, 4, 'Elsabe Human', 'elsabe.human@karoofreight.co.za'),
       (79, 4, 'Kyle Pieterse', 'kyle.pieterse@karoofreight.co.za'),
       (80, 4, 'Dineo Masilela', 'dineo.masilela@karoofreight.co.za'),
       (81, 4, 'Byron September', 'byron.september@karoofreight.co.za'),
       (82, 4, 'Tanya Engelbrecht', 'tanya.engelbrecht@karoofreight.co.za'),
       (83, 4, 'Sandile Xaba', 'sandile.xaba@karoofreight.co.za'),
       (84, 4, 'Reza Ebrahim', 'reza.ebrahim@karoofreight.co.za'),
       (85, 4, 'Martie Labuschagne', 'martie.labuschagne@karoofreight.co.za'),
       (86, 4, 'Tebogo Moloi', 'tebogo.moloi@karoofreight.co.za'),
       (87, 4, 'Ashley Fortuin', 'ashley.fortuin@karoofreight.co.za'),
       (88, 4, 'Sizwe Majola', 'sizwe.majola@karoofreight.co.za'),
       (89, 4, 'Lorraine van Wyk', 'lorraine.vanwyk@karoofreight.co.za'),
       (90, 4, 'Nkosinathi Zungu', 'nkosinathi.zungu@karoofreight.co.za'),
       (91, 4, 'Heinrich Erasmus', 'heinrich.erasmus@karoofreight.co.za'),
       (92, 4, 'Kamogelo Sebola', 'kamogelo.sebola@karoofreight.co.za'),
       (93, 4, 'Jolene Marais', 'jolene.marais@karoofreight.co.za'),
       (94, 4, 'Ntando Msimang', 'ntando.msimang@karoofreight.co.za'),
       (95, 4, 'Yusuf Mohamed', 'yusuf.mohamed@karoofreight.co.za'),
       (96, 4, 'Christo Barnard', 'christo.barnard@karoofreight.co.za'),
       (97, 4, 'Lebo Maseko', 'lebo.maseko@karoofreight.co.za'),
       (98, 4, 'Bianca Carelse', 'bianca.carelse@karoofreight.co.za'),
       (99, 4, 'Siyabonga Mazibuko', 'siyabonga.mazibuko@karoofreight.co.za'),
       (100, 4, 'Dirk Viljoen', 'dirk.viljoen@karoofreight.co.za'),
       (101, 4, 'Mapule Sekhukhune', 'mapule.sekhukhune@karoofreight.co.za'),
       (102, 4, 'Rowan Jansen', 'rowan.jansen@karoofreight.co.za'),
       (103, 4, 'Gugu Nxumalo', 'gugu.nxumalo@karoofreight.co.za'),
       (104, 4, 'Pravesh Maharaj', 'pravesh.maharaj@karoofreight.co.za'),
       (105, 4, 'Retha Cilliers', 'retha.cilliers@karoofreight.co.za'),
       (106, 4, 'Kabelo Modise', 'kabelo.modise@karoofreight.co.za'),
       (107, 4, 'Lauren Fisher', 'lauren.fisher@karoofreight.co.za'),
       (108, 4, 'Mxolisi Shezi', 'mxolisi.shezi@karoofreight.co.za'),
       (109, 4, 'Johannes Kotze', 'johannes.kotze@karoofreight.co.za'),
       (110, 4, 'Boitumelo Phiri', 'boitumelo.phiri@karoofreight.co.za'),
       (111, 4, 'Clint Wessels', 'clint.wessels@karoofreight.co.za'),
       (112, 4, 'Zinhle Mkhabela', 'zinhle.mkhabela@karoofreight.co.za'),
       (113, 4, 'Nishal Ramlall', 'nishal.ramlall@karoofreight.co.za'),
       (114, 4, 'Adriaan Muller', 'adriaan.muller@karoofreight.co.za'),
       (115, 4, 'Thabiso Lekota', 'thabiso.lekota@karoofreight.co.za'),
       (116, 4, 'Nicole February', 'nicole.february@karoofreight.co.za'),
       (117, 4, 'Musa Sibiya', 'musa.sibiya@karoofreight.co.za'),
       (118, 4, 'Wian Smuts', 'wian.smuts@karoofreight.co.za'),
       (119, 4, 'Karabo Tshabalala', 'karabo.tshabalala@karoofreight.co.za'),
       (120, 4, 'Shaun Arendse', 'shaun.arendse@karoofreight.co.za'),
       (121, 4, 'Sanele Mthethwa', 'sanele.mthethwa@karoofreight.co.za'),
       (122, 4, 'Hannelie Brink', 'hannelie.brink@karoofreight.co.za'),
       (123, 4, 'Olwethu Mhlongo', 'olwethu.mhlongo@karoofreight.co.za'),
       (124, 4, 'Imraan Parker', 'imraan.parker@karoofreight.co.za'),
       (125, 4, 'Frikkie Loubser', 'frikkie.loubser@karoofreight.co.za'),
       (126, 4, 'Tshegofatso Ramokgopa', 'tshegofatso.ramokgopa@karoofreight.co.za'),
       (127, 4, 'Jenna Walker', 'jenna.walker@karoofreight.co.za'),
       (128, 4, 'Ayabonga Nqaba', 'ayabonga.nqaba@karoofreight.co.za'),
       (129, 4, 'Pierre Theron', 'pierre.theron@karoofreight.co.za'),
       (130, 4, 'Mbali Hlongwane', 'mbali.hlongwane@karoofreight.co.za'),
       (131, 4, 'Sunil Bhana', 'sunil.bhana@karoofreight.co.za'),
       (132, 4, 'Arno Vermeulen', 'arno.vermeulen@karoofreight.co.za'),
       (133, 4, 'Lindokuhle Shange', 'lindokuhle.shange@karoofreight.co.za'),
       (134, 4, 'Ricardo Lottering', 'ricardo.lottering@karoofreight.co.za'),
       (135, 4, 'Masego Kgosana', 'masego.kgosana@karoofreight.co.za'),
       (136, 4, 'Tiaan Blom', 'tiaan.blom@karoofreight.co.za'),
       (137, 4, 'Nandi Madlala', 'nandi.madlala@karoofreight.co.za'),
       (138, 4, 'Keegan Solomons', 'keegan.solomons@karoofreight.co.za'),
       (139, 4, 'Thapelo Makgoba', 'thapelo.makgoba@karoofreight.co.za'),
       (140, 4, 'Leonie Grobler', 'leonie.grobler@karoofreight.co.za'),
       (141, 4, 'Bheki Cebekhulu', 'bheki.cebekhulu@karoofreight.co.za'),
       (142, 4, 'Denise Arnolds', 'denise.arnolds@karoofreight.co.za'),
       (143, 4, 'Kgomotso Seleka', 'kgomotso.seleka@karoofreight.co.za'),
       (144, 4, 'Morne Strydom', 'morne.strydom@karoofreight.co.za'),
       (145, 4, 'Ntombi Khanyile', 'ntombi.khanyile@karoofreight.co.za'),
       (146, 4, 'Faizel Samuels', 'faizel.samuels@karoofreight.co.za'),
       (147, 4, 'Hanno Burger', 'hanno.burger@karoofreight.co.za'),
       (148, 4, 'Onalenna Kgari', 'onalenna.kgari@karoofreight.co.za'),
       (149, 4, 'Cheslyn Adonis', 'cheslyn.adonis@karoofreight.co.za'),
       (150, 4, 'Vuyo Mbatha', 'vuyo.mbatha@karoofreight.co.za'),
       (151, 4, 'Riana Fouche', 'riana.fouche@karoofreight.co.za'),
       (152, 4, 'Tumelo Mathebula', 'tumelo.mathebula@karoofreight.co.za'),
       (153, 4, 'Duncan Frazer', 'duncan.frazer@karoofreight.co.za'),
       (154, 4, 'Khanyisile Dlomo', 'khanyisile.dlomo@karoofreight.co.za'),
       (155, 4, 'Werner Horn', 'werner.horn@karoofreight.co.za'),
       (156, 4, 'Lesedi Moagi', 'lesedi.moagi@karoofreight.co.za'),
       (157, 4, 'Zaid Hassiem', 'zaid.hassiem@karoofreight.co.za');

-- Fiona Marsh Bookkeeping, a sole trader with one person.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (5, 'Fiona Marsh Bookkeeping');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (158, 5, 'Fiona Marsh', 'fiona@marshbookkeeping.co.za');

-- Umhlanga Physio, an organisation with no people.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (6, 'Umhlanga Physio');

-- Karoo Group Holdings and Karoo Cold Chain are two separate organisations. Their people share
-- the karoogroup.co.za domain.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (7, 'Karoo Group Holdings');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (159, 7, 'Marius Oberholzer', 'marius.oberholzer@karoogroup.co.za'),
       (160, 7, 'Tracey-Lee Daniels', 'tracey-lee.daniels@karoogroup.co.za'),
       (161, 7, 'Siphesihle Ngema', 'siphesihle.ngema@karoogroup.co.za');

INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (8, 'Karoo Cold Chain');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (162, 8, 'Anton Steenkamp', 'anton.steenkamp@karoogroup.co.za'),
       (163, 8, 'Lindiwe Zikalala', 'lindiwe.zikalala@karoogroup.co.za'),
       (164, 8, 'Faheem Dawood', 'faheem.dawood@karoogroup.co.za'),
       (165, 8, 'Carmen Titus', 'carmen.titus@karoogroup.co.za');

-- Ma'Khumalo's Kitchen & Café. Its name contains two apostrophes, an ampersand and an accented
-- letter. SQL writes each apostrophe inside a string as two. Its staff use first-name emails.
INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (9, 'Ma''Khumalo''s Kitchen & Café');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (166, 9, 'Busi Khumalo', 'busi@makhumaloskitchen.co.za'),
       (167, 9, 'Elrico Julies', 'elrico@makhumaloskitchen.co.za'),
       (168, 9, 'Nomsa Mahlaba', 'nomsa@makhumaloskitchen.co.za');

INSERT
INTO ticket_type (ticket_type_id, name)
OVERRIDING SYSTEM VALUE
VALUES (1, 'Incident'),
       (2, 'Service Request'),
       (3, 'Problem'),
       (4, 'Change'),
       (5, 'Question'),
       (6, 'Hardware & Peripherals'),
       (7, 'Onboarding');

INSERT
INTO ticket_status (status_id, name)
OVERRIDING SYSTEM VALUE
VALUES (1, 'New'),
       (2, 'In Progress'),
       (3, 'Waiting on Customer'),
       (4, 'Waiting on Third Party'),
       (5, 'Resolved'),
       (6, 'Closed');

-- PostgreSQL keeps a counter for each id column and takes the next id from it whenever an INSERT
-- leaves the id out. An INSERT that states its id leaves that counter at its starting value. Each
-- statement below therefore moves one counter to the highest id in its table. The next row a test
-- inserts into that table then takes the id after that one.
SELECT setval(pg_get_serial_sequence('org', 'org_id'), (SELECT max(org_id) FROM org));
SELECT setval(pg_get_serial_sequence('person', 'person_id'), (SELECT max(person_id) FROM person));
SELECT setval(pg_get_serial_sequence('ticket_type', 'ticket_type_id'), (SELECT max(ticket_type_id) FROM ticket_type));
SELECT setval(pg_get_serial_sequence('ticket_status', 'status_id'), (SELECT max(status_id) FROM ticket_status));
