INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (1, 'Slinky IT');

INSERT
INTO tenant (org_id)
VALUES (1);

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
       (130, 4, 'Mbali Hlongwane', 'mbali.hlongwane@outlook.com'),
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
       (155, 4, 'Werner Horn', 'werner.horn@gmail.com'),
       (156, 4, 'Lesedi Moagi', 'lesedi.moagi@karoofreight.co.za'),
       (157, 4, 'Zaid Hassiem', 'zaid.hassiem@karoofreight.co.za');

INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (5, 'Fiona Marsh Bookkeeping');

INSERT
INTO person (person_id, org_id, name, email)
OVERRIDING SYSTEM VALUE
VALUES (158, 5, 'Fiona Marsh', 'fiona@marshbookkeeping.co.za');

INSERT
INTO org (org_id, name)
OVERRIDING SYSTEM VALUE
VALUES (6, 'Umhlanga Physio');

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
       (7, 'Onboarding'),
       (8, 'Licensing');

INSERT
INTO ticket_status (status_id, name)
OVERRIDING SYSTEM VALUE
VALUES (1, 'New'),
       (2, 'In Progress'),
       (3, 'Waiting on Customer'),
       (4, 'Waiting on Third Party'),
       (5, 'Resolved'),
       (6, 'Closed');

INSERT
INTO ticket (ticket_id, ticket_type_id, status_id, priority, title, raised_by, created_at, closed_at, description)
OVERRIDING SYSTEM VALUE
VALUES (1, 1, 1, 216, 'Test Ticket 1', 100, TIMESTAMPTZ '2026-01-09 16:00:00+00', NULL, 'Test Ticket 1'),
       (2, 1, 2, 220, 'Test Ticket 2', 48, TIMESTAMPTZ '2026-01-12 11:00:00+00', NULL, 'Test Ticket 2'),
       (3, 1, 3, 54, 'Test Ticket 3', 143, TIMESTAMPTZ '2026-01-06 16:00:00+00', NULL, 'Test Ticket 3'),
       (4, 1, 4, 66, 'Test Ticket 4', 91, TIMESTAMPTZ '2026-01-10 14:00:00+00', NULL, 'Test Ticket 4'),
       (5, 1, 5, 176, 'Test Ticket 5', 22, TIMESTAMPTZ '2026-01-18 09:00:00+00', NULL, 'Test Ticket 5'),
       (6, 1, 6, 35, 'Test Ticket 6', 20, TIMESTAMPTZ '2026-01-10 16:00:00+00', TIMESTAMPTZ '2026-01-14 17:00:00+00', 'Test Ticket 6'),
       (7, 2, 1, 226, 'Test Ticket 7', 31, TIMESTAMPTZ '2026-01-20 10:00:00+00', NULL, 'Test Ticket 7'),
       (8, 2, 2, 221, 'Test Ticket 8', 85, TIMESTAMPTZ '2026-01-10 10:00:00+00', NULL, 'Test Ticket 8'),
       (9, 2, 3, 121, 'Test Ticket 9', 167, TIMESTAMPTZ '2026-01-15 16:00:00+00', NULL, 'Test Ticket 9'),
       (10, 2, 4, 43, 'Test Ticket 10', 160, TIMESTAMPTZ '2026-01-06 09:00:00+00', NULL, 'Test Ticket 10'),
       (11, 2, 5, 225, 'Test Ticket 11', 68, TIMESTAMPTZ '2026-01-05 12:00:00+00', NULL, 'Test Ticket 11'),
       (12, 2, 6, 27, 'Test Ticket 12', 76, TIMESTAMPTZ '2026-01-19 08:00:00+00', TIMESTAMPTZ '2026-01-25 17:00:00+00', 'Test Ticket 12'),
       (13, 3, 1, 244, 'Test Ticket 13', 55, TIMESTAMPTZ '2026-01-04 15:00:00+00', NULL, 'Test Ticket 13'),
       (14, 3, 2, 118, 'Test Ticket 14', 141, TIMESTAMPTZ '2026-01-06 09:00:00+00', NULL, 'Test Ticket 14'),
       (15, 3, 3, 126, 'Test Ticket 15', 34, TIMESTAMPTZ '2026-01-03 08:00:00+00', NULL, 'Test Ticket 15'),
       (16, 3, 4, 108, 'Test Ticket 16', 48, TIMESTAMPTZ '2026-01-14 10:00:00+00', NULL, 'Test Ticket 16'),
       (17, 3, 5, 142, 'Test Ticket 17', 29, TIMESTAMPTZ '2026-01-12 08:00:00+00', NULL, 'Test Ticket 17'),
       (18, 3, 6, 46, 'Test Ticket 18', 96, TIMESTAMPTZ '2026-01-15 10:00:00+00', TIMESTAMPTZ '2026-01-16 17:00:00+00', 'Test Ticket 18'),
       (19, 4, 1, 112, 'Test Ticket 19', 161, TIMESTAMPTZ '2026-01-20 09:00:00+00', NULL, 'Test Ticket 19'),
       (20, 4, 2, 6, 'Test Ticket 20', 112, TIMESTAMPTZ '2026-01-12 15:00:00+00', NULL, 'Test Ticket 20'),
       (21, 4, 3, 38, 'Test Ticket 21', 104, TIMESTAMPTZ '2026-01-01 16:00:00+00', NULL, 'Test Ticket 21'),
       (22, 4, 4, 20, 'Test Ticket 22', 74, TIMESTAMPTZ '2026-01-14 12:00:00+00', NULL, 'Test Ticket 22'),
       (23, 4, 5, 45, 'Test Ticket 23', 157, TIMESTAMPTZ '2026-01-02 11:00:00+00', NULL, 'Test Ticket 23'),
       (24, 4, 6, 111, 'Test Ticket 24', 111, TIMESTAMPTZ '2026-01-04 10:00:00+00', TIMESTAMPTZ '2026-01-05 17:00:00+00', 'Test Ticket 24'),
       (25, 5, 1, 211, 'Test Ticket 25', 115, TIMESTAMPTZ '2026-01-12 13:00:00+00', NULL, 'Test Ticket 25'),
       (26, 5, 2, 253, 'Test Ticket 26', 125, TIMESTAMPTZ '2026-01-07 13:00:00+00', NULL, 'Test Ticket 26'),
       (27, 5, 3, 251, 'Test Ticket 27', 168, TIMESTAMPTZ '2026-01-20 16:00:00+00', NULL, 'Test Ticket 27'),
       (28, 5, 4, 201, 'Test Ticket 28', 167, TIMESTAMPTZ '2026-01-15 10:00:00+00', NULL, 'Test Ticket 28'),
       (29, 5, 5, 77, 'Test Ticket 29', 124, TIMESTAMPTZ '2026-01-13 11:00:00+00', NULL, 'Test Ticket 29'),
       (30, 5, 6, 152, 'Test Ticket 30', 142, TIMESTAMPTZ '2026-01-13 16:00:00+00', TIMESTAMPTZ '2026-01-16 17:00:00+00', 'Test Ticket 30'),
       (31, 6, 1, 45, 'Test Ticket 31', 115, TIMESTAMPTZ '2026-01-15 13:00:00+00', NULL, 'Test Ticket 31'),
       (32, 6, 2, 90, 'Test Ticket 32', 25, TIMESTAMPTZ '2026-01-10 10:00:00+00', NULL, 'Test Ticket 32'),
       (33, 6, 3, 30, 'Test Ticket 33', 80, TIMESTAMPTZ '2026-01-17 11:00:00+00', NULL, 'Test Ticket 33'),
       (34, 6, 4, 238, 'Test Ticket 34', 71, TIMESTAMPTZ '2026-01-06 10:00:00+00', NULL, 'Test Ticket 34'),
       (35, 6, 5, 77, 'Test Ticket 35', 106, TIMESTAMPTZ '2026-01-02 10:00:00+00', NULL, 'Test Ticket 35'),
       (36, 6, 6, 159, 'Test Ticket 36', 152, TIMESTAMPTZ '2026-01-04 13:00:00+00', TIMESTAMPTZ '2026-01-10 17:00:00+00', 'Test Ticket 36'),
       (37, 7, 1, 142, 'Test Ticket 37', 74, TIMESTAMPTZ '2026-01-13 14:00:00+00', NULL, 'Test Ticket 37'),
       (38, 7, 2, 128, 'Test Ticket 38', 159, TIMESTAMPTZ '2026-01-06 09:00:00+00', NULL, 'Test Ticket 38'),
       (39, 7, 3, 70, 'Test Ticket 39', 116, TIMESTAMPTZ '2026-01-11 08:00:00+00', NULL, 'Test Ticket 39'),
       (40, 7, 4, 39, 'Test Ticket 40', 33, TIMESTAMPTZ '2026-01-06 16:00:00+00', NULL, 'Test Ticket 40'),
       (41, 7, 5, 146, 'Test Ticket 41', 123, TIMESTAMPTZ '2026-01-03 16:00:00+00', NULL, 'Test Ticket 41'),
       (42, 7, 6, 119, 'Test Ticket 42', 89, TIMESTAMPTZ '2026-01-18 11:00:00+00', TIMESTAMPTZ '2026-01-25 17:00:00+00', 'Test Ticket 42'),
       (43, 8, 1, 10, 'Test Ticket 43', 26, TIMESTAMPTZ '2026-01-07 15:00:00+00', NULL, 'Test Ticket 43'),
       (44, 8, 2, 100, 'Test Ticket 44', 162, TIMESTAMPTZ '2026-01-04 16:00:00+00', NULL, 'Test Ticket 44'),
       (45, 8, 3, 165, 'Test Ticket 45', 35, TIMESTAMPTZ '2026-01-09 08:00:00+00', NULL, 'Test Ticket 45'),
       (46, 8, 4, 125, 'Test Ticket 46', 20, TIMESTAMPTZ '2026-01-05 13:00:00+00', NULL, 'Test Ticket 46'),
       (47, 8, 5, 246, 'Test Ticket 47', 125, TIMESTAMPTZ '2026-01-13 15:00:00+00', NULL, 'Test Ticket 47'),
       (48, 8, 6, 16, 'Test Ticket 48', 163, TIMESTAMPTZ '2026-01-04 14:00:00+00', TIMESTAMPTZ '2026-01-06 17:00:00+00', 'Test Ticket 48');

SELECT setval(pg_get_serial_sequence('org', 'org_id'), (SELECT max(org_id) FROM org));
SELECT setval(pg_get_serial_sequence('person', 'person_id'), (SELECT max(person_id) FROM person));
SELECT setval(pg_get_serial_sequence('ticket_type', 'ticket_type_id'), (SELECT max(ticket_type_id) FROM ticket_type));
SELECT setval(pg_get_serial_sequence('ticket_status', 'status_id'), (SELECT max(status_id) FROM ticket_status));
SELECT setval(pg_get_serial_sequence('ticket', 'ticket_id'), (SELECT max(ticket_id) FROM ticket));
