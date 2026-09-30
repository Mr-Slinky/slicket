-- The happy path fixture fills a migrated database with a tenant and the organisations it serves.
-- The tenant is Slinky IT, and each organisation after it has its own set of people.
--
-- Each organisation takes one statement. Its WITH block inserts the org row and returns the new
-- org_id. The INSERT beneath it then gives that org_id to each of the organisation's people.
--
-- The people come from two arrays, one of names and one of emails. unnest pairs the name at each
-- position with the email at the same position, and WITH ORDINALITY numbers each pair by that
-- position. ORDER BY inserts the pairs in that order, meaning person_id follows the order of the
-- lists. The two arrays must be the same length, since unnest pads the shorter one with NULL and
-- the person table rejects a NULL name or email.
--
-- Every email in the file is unique, ignoring case. The person_email_idx index on person rejects
-- a second copy of an address.

-- Slinky IT, the tenant. The tenant table stores the org_id of this one org row.
WITH tenant_org AS (
    INSERT
    INTO org (name)
    VALUES ('Slinky IT')
    RETURNING org_id
)
INSERT
INTO tenant (org_id)
SELECT org_id
FROM tenant_org;

-- Slinky IT's 15 staff, all on slicket.com. This statement reads the org_id from the tenant row.
INSERT
INTO person (org_id, name, email)
SELECT tenant.org_id, staff.name, staff.email
FROM tenant,
     unnest(
         ARRAY[
             'Thandiwe Nkosi',
             'Pieter van der Merwe',
             'Aisha Patel',
             'Sipho Dlamini',
             'Megan O''Connor',
             'Lerato Mokoena',
             'Johan Botha',
             'Priya Naidoo',
             'Kagiso Molefe',
             'Chloe Adams',
             'Tshepo Mahlangu',
             'Ruan Steyn',
             'Nomvula Zulu',
             'Daniel Fourie',
             'Zanele Khumalo'
         ],
         ARRAY[
             'thandiwe.nkosi@slicket.com',
             'pieter.vandermerwe@slicket.com',
             'aisha.patel@slicket.com',
             'sipho.dlamini@slicket.com',
             'megan.oconnor@slicket.com',
             'lerato.mokoena@slicket.com',
             'johan.botha@slicket.com',
             'priya.naidoo@slicket.com',
             'kagiso.molefe@slicket.com',
             'chloe.adams@slicket.com',
             'tshepo.mahlangu@slicket.com',
             'ruan.steyn@slicket.com',
             'nomvula.zulu@slicket.com',
             'daniel.fourie@slicket.com',
             'zanele.khumalo@slicket.com'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

-- Harbourview Dental, a small practice with 12 staff on its own domain.
WITH harbourview AS (
    INSERT
    INTO org (name)
    VALUES ('Harbourview Dental')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT harbourview.org_id, staff.name, staff.email
FROM harbourview,
     unnest(
         ARRAY[
             'Nadia Hendricks',
             'Craig Williams',
             'Fatima Isaacs',
             'Bongani Ndlovu',
             'Liezl Pretorius',
             'Ayanda Mthembu',
             'Shannon Jacobs',
             'Riaan du Plessis',
             'Kavitha Govender',
             'Lindiwe Sithole',
             'Marco Ferreira',
             'Chloe Adams'
         ],
         ARRAY[
             'nadia.hendricks@harbourviewdental.co.za',
             'craig.williams@harbourviewdental.co.za',
             'fatima.isaacs@harbourviewdental.co.za',
             'bongani.ndlovu@harbourviewdental.co.za',
             'liezl.pretorius@harbourviewdental.co.za',
             'ayanda.mthembu@harbourviewdental.co.za',
             'shannon.jacobs@harbourviewdental.co.za',
             'riaan.duplessis@harbourviewdental.co.za',
             'kavitha.govender@harbourviewdental.co.za',
             'lindiwe.sithole@harbourviewdental.co.za',
             'marco.ferreira@harbourviewdental.co.za',
             'chloe.adams@harbourviewdental.co.za'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

-- Oakridge Primary School, a medium-sized school with 30 staff on personal gmail.com addresses.
WITH oakridge AS (
    INSERT
    INTO org (name)
    VALUES ('Oakridge Primary School')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT oakridge.org_id, staff.name, staff.email
FROM oakridge,
     unnest(
         ARRAY[
             'Elaine Visser',
             'Themba Cele',
             'Rochelle Petersen',
             'Sipho Dlamini',
             'Karen Smit',
             'Mandla Shabalala',
             'Natasha Moodley',
             'Gert Swanepoel',
             'Palesa Mofokeng',
             'Brendan Lewis',
             'Nokuthula Dube',
             'Anita Reddy',
             'Willem Joubert',
             'Zodwa Ngcobo',
             'Candice Abrahams',
             'Lucky Maluleke',
             'Marietjie Kruger',
             'Sibusiso Mkhize',
             'Tamsin Clarke',
             'Refilwe Motaung',
             'Deon Oosthuizen',
             'Hlengiwe Buthelezi',
             'Samantha Davids',
             'Vusi Radebe',
             'Charmaine Olivier',
             'Neo Seabi',
             'Ilse Venter',
             'Ashwin Pillay',
             'Busisiwe Khoza',
             'Gavin Meyer'
         ],
         ARRAY[
             'elaine.visser@gmail.com',
             'themba.cele@gmail.com',
             'rochelle.petersen@gmail.com',
             'sipho.dlamini@gmail.com',
             'karen.smit@gmail.com',
             'mandla.shabalala@gmail.com',
             'natasha.moodley@gmail.com',
             'gert.swanepoel@gmail.com',
             'palesa.mofokeng@gmail.com',
             'brendan.lewis@gmail.com',
             'nokuthula.dube@gmail.com',
             'anita.reddy@gmail.com',
             'willem.joubert@gmail.com',
             'zodwa.ngcobo@gmail.com',
             'candice.abrahams@gmail.com',
             'lucky.maluleke@gmail.com',
             'marietjie.kruger@gmail.com',
             'sibusiso.mkhize@gmail.com',
             'tamsin.clarke@gmail.com',
             'refilwe.motaung@gmail.com',
             'deon.oosthuizen@gmail.com',
             'hlengiwe.buthelezi@gmail.com',
             'samantha.davids@gmail.com',
             'vusi.radebe@gmail.com',
             'charmaine.olivier@gmail.com',
             'neo.seabi@gmail.com',
             'ilse.venter@gmail.com',
             'ashwin.pillay@gmail.com',
             'busisiwe.khoza@gmail.com',
             'gavin.meyer@gmail.com'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

-- Karoo Freight, a large company with 100 employees on karoofreight.co.za.
WITH karoo AS (
    INSERT
    INTO org (name)
    VALUES ('Karoo Freight')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT karoo.org_id, staff.name, staff.email
FROM karoo,
     unnest(
         ARRAY[
             'Hendrik Coetzee',
             'Nompumelelo Mabaso',
             'Jason Naicker',
             'Carla Bezuidenhout',
             'Lwazi Hadebe',
             'Stefan Nel',
             'Precious Mashaba',
             'Ravi Singh',
             'Annelie du Toit',
             'Mpho Tau',
             'Grant Robertson',
             'Thuli Mnguni',
             'Riyaad Salie',
             'Jacques Lombard',
             'Nosipho Gumede',
             'Wayne Hartley',
             'Keabetsoe Letsie',
             'Francois Rossouw',
             'Shireen Kader',
             'Mzwandile Ntuli',
             'Elsabe Human',
             'Kyle Pieterse',
             'Dineo Masilela',
             'Byron September',
             'Tanya Engelbrecht',
             'Sandile Xaba',
             'Reza Ebrahim',
             'Martie Labuschagne',
             'Tebogo Moloi',
             'Ashley Fortuin',
             'Sizwe Majola',
             'Lorraine van Wyk',
             'Nkosinathi Zungu',
             'Heinrich Erasmus',
             'Kamogelo Sebola',
             'Jolene Marais',
             'Ntando Msimang',
             'Yusuf Mohamed',
             'Christo Barnard',
             'Lebo Maseko',
             'Bianca Carelse',
             'Siyabonga Mazibuko',
             'Dirk Viljoen',
             'Mapule Sekhukhune',
             'Rowan Jansen',
             'Gugu Nxumalo',
             'Pravesh Maharaj',
             'Retha Cilliers',
             'Kabelo Modise',
             'Lauren Fisher',
             'Mxolisi Shezi',
             'Johannes Kotze',
             'Boitumelo Phiri',
             'Clint Wessels',
             'Zinhle Mkhabela',
             'Nishal Ramlall',
             'Adriaan Muller',
             'Thabiso Lekota',
             'Nicole February',
             'Musa Sibiya',
             'Wian Smuts',
             'Karabo Tshabalala',
             'Shaun Arendse',
             'Sanele Mthethwa',
             'Hannelie Brink',
             'Olwethu Mhlongo',
             'Imraan Parker',
             'Frikkie Loubser',
             'Tshegofatso Ramokgopa',
             'Jenna Walker',
             'Ayabonga Nqaba',
             'Pierre Theron',
             'Mbali Hlongwane',
             'Sunil Bhana',
             'Arno Vermeulen',
             'Lindokuhle Shange',
             'Ricardo Lottering',
             'Masego Kgosana',
             'Tiaan Blom',
             'Nandi Madlala',
             'Keegan Solomons',
             'Thapelo Makgoba',
             'Leonie Grobler',
             'Bheki Cebekhulu',
             'Denise Arnolds',
             'Kgomotso Seleka',
             'Morne Strydom',
             'Ntombi Khanyile',
             'Faizel Samuels',
             'Hanno Burger',
             'Onalenna Kgari',
             'Cheslyn Adonis',
             'Vuyo Mbatha',
             'Riana Fouche',
             'Tumelo Mathebula',
             'Duncan Frazer',
             'Khanyisile Dlomo',
             'Werner Horn',
             'Lesedi Moagi',
             'Zaid Hassiem'
         ],
         ARRAY[
             'hendrik.coetzee@karoofreight.co.za',
             'nompumelelo.mabaso@karoofreight.co.za',
             'jason.naicker@karoofreight.co.za',
             'carla.bezuidenhout@karoofreight.co.za',
             'lwazi.hadebe@karoofreight.co.za',
             'stefan.nel@karoofreight.co.za',
             'precious.mashaba@karoofreight.co.za',
             'ravi.singh@karoofreight.co.za',
             'annelie.dutoit@karoofreight.co.za',
             'mpho.tau@karoofreight.co.za',
             'grant.robertson@karoofreight.co.za',
             'thuli.mnguni@karoofreight.co.za',
             'riyaad.salie@karoofreight.co.za',
             'jacques.lombard@karoofreight.co.za',
             'nosipho.gumede@karoofreight.co.za',
             'wayne.hartley@karoofreight.co.za',
             'keabetsoe.letsie@karoofreight.co.za',
             'francois.rossouw@karoofreight.co.za',
             'shireen.kader@karoofreight.co.za',
             'mzwandile.ntuli@karoofreight.co.za',
             'elsabe.human@karoofreight.co.za',
             'kyle.pieterse@karoofreight.co.za',
             'dineo.masilela@karoofreight.co.za',
             'byron.september@karoofreight.co.za',
             'tanya.engelbrecht@karoofreight.co.za',
             'sandile.xaba@karoofreight.co.za',
             'reza.ebrahim@karoofreight.co.za',
             'martie.labuschagne@karoofreight.co.za',
             'tebogo.moloi@karoofreight.co.za',
             'ashley.fortuin@karoofreight.co.za',
             'sizwe.majola@karoofreight.co.za',
             'lorraine.vanwyk@karoofreight.co.za',
             'nkosinathi.zungu@karoofreight.co.za',
             'heinrich.erasmus@karoofreight.co.za',
             'kamogelo.sebola@karoofreight.co.za',
             'jolene.marais@karoofreight.co.za',
             'ntando.msimang@karoofreight.co.za',
             'yusuf.mohamed@karoofreight.co.za',
             'christo.barnard@karoofreight.co.za',
             'lebo.maseko@karoofreight.co.za',
             'bianca.carelse@karoofreight.co.za',
             'siyabonga.mazibuko@karoofreight.co.za',
             'dirk.viljoen@karoofreight.co.za',
             'mapule.sekhukhune@karoofreight.co.za',
             'rowan.jansen@karoofreight.co.za',
             'gugu.nxumalo@karoofreight.co.za',
             'pravesh.maharaj@karoofreight.co.za',
             'retha.cilliers@karoofreight.co.za',
             'kabelo.modise@karoofreight.co.za',
             'lauren.fisher@karoofreight.co.za',
             'mxolisi.shezi@karoofreight.co.za',
             'johannes.kotze@karoofreight.co.za',
             'boitumelo.phiri@karoofreight.co.za',
             'clint.wessels@karoofreight.co.za',
             'zinhle.mkhabela@karoofreight.co.za',
             'nishal.ramlall@karoofreight.co.za',
             'adriaan.muller@karoofreight.co.za',
             'thabiso.lekota@karoofreight.co.za',
             'nicole.february@karoofreight.co.za',
             'musa.sibiya@karoofreight.co.za',
             'wian.smuts@karoofreight.co.za',
             'karabo.tshabalala@karoofreight.co.za',
             'shaun.arendse@karoofreight.co.za',
             'sanele.mthethwa@karoofreight.co.za',
             'hannelie.brink@karoofreight.co.za',
             'olwethu.mhlongo@karoofreight.co.za',
             'imraan.parker@karoofreight.co.za',
             'frikkie.loubser@karoofreight.co.za',
             'tshegofatso.ramokgopa@karoofreight.co.za',
             'jenna.walker@karoofreight.co.za',
             'ayabonga.nqaba@karoofreight.co.za',
             'pierre.theron@karoofreight.co.za',
             'mbali.hlongwane@karoofreight.co.za',
             'sunil.bhana@karoofreight.co.za',
             'arno.vermeulen@karoofreight.co.za',
             'lindokuhle.shange@karoofreight.co.za',
             'ricardo.lottering@karoofreight.co.za',
             'masego.kgosana@karoofreight.co.za',
             'tiaan.blom@karoofreight.co.za',
             'nandi.madlala@karoofreight.co.za',
             'keegan.solomons@karoofreight.co.za',
             'thapelo.makgoba@karoofreight.co.za',
             'leonie.grobler@karoofreight.co.za',
             'bheki.cebekhulu@karoofreight.co.za',
             'denise.arnolds@karoofreight.co.za',
             'kgomotso.seleka@karoofreight.co.za',
             'morne.strydom@karoofreight.co.za',
             'ntombi.khanyile@karoofreight.co.za',
             'faizel.samuels@karoofreight.co.za',
             'hanno.burger@karoofreight.co.za',
             'onalenna.kgari@karoofreight.co.za',
             'cheslyn.adonis@karoofreight.co.za',
             'vuyo.mbatha@karoofreight.co.za',
             'riana.fouche@karoofreight.co.za',
             'tumelo.mathebula@karoofreight.co.za',
             'duncan.frazer@karoofreight.co.za',
             'khanyisile.dlomo@karoofreight.co.za',
             'werner.horn@karoofreight.co.za',
             'lesedi.moagi@karoofreight.co.za',
             'zaid.hassiem@karoofreight.co.za'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

-- Fiona Marsh Bookkeeping, a sole trader. Its one person is inserted from literal values, with no
-- arrays.
WITH marsh AS (
    INSERT
    INTO org (name)
    VALUES ('Fiona Marsh Bookkeeping')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT marsh.org_id, 'Fiona Marsh', 'fiona@marshbookkeeping.co.za'
FROM marsh;

-- Umhlanga Physio, an organisation with no people.
INSERT
INTO org (name)
VALUES ('Umhlanga Physio');

-- Karoo Group Holdings and Karoo Cold Chain are two separate organisations. Their people share
-- the karoogroup.co.za domain.
WITH karoo_group AS (
    INSERT
    INTO org (name)
    VALUES ('Karoo Group Holdings')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT karoo_group.org_id, staff.name, staff.email
FROM karoo_group,
     unnest(
         ARRAY[
             'Marius Oberholzer',
             'Tracey-Lee Daniels',
             'Siphesihle Ngema'
         ],
         ARRAY[
             'marius.oberholzer@karoogroup.co.za',
             'tracey-lee.daniels@karoogroup.co.za',
             'siphesihle.ngema@karoogroup.co.za'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

WITH karoo_cold AS (
    INSERT
    INTO org (name)
    VALUES ('Karoo Cold Chain')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT karoo_cold.org_id, staff.name, staff.email
FROM karoo_cold,
     unnest(
         ARRAY[
             'Anton Steenkamp',
             'Lindiwe Zikalala',
             'Faheem Dawood',
             'Carmen Titus'
         ],
         ARRAY[
             'anton.steenkamp@karoogroup.co.za',
             'lindiwe.zikalala@karoogroup.co.za',
             'faheem.dawood@karoogroup.co.za',
             'carmen.titus@karoogroup.co.za'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

-- Ma'Khumalo's Kitchen & Café. Its name contains two apostrophes, an ampersand and an accented
-- letter. SQL writes each apostrophe inside a string as two. Its staff use first-name emails.
WITH makhumalo AS (
    INSERT
    INTO org (name)
    VALUES ('Ma''Khumalo''s Kitchen & Café')
    RETURNING org_id
)
INSERT
INTO person (org_id, name, email)
SELECT makhumalo.org_id, staff.name, staff.email
FROM makhumalo,
     unnest(
         ARRAY[
             'Busi Khumalo',
             'Elrico Julies',
             'Nomsa Mahlaba'
         ],
         ARRAY[
             'busi@makhumaloskitchen.co.za',
             'elrico@makhumaloskitchen.co.za',
             'nomsa@makhumaloskitchen.co.za'
         ]
     ) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;
