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

INSERT INTO person (org_id, name, email)
SELECT tenant.org_id, staff.name, staff.email
FROM tenant,
     unnest(ARRAY[
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
                ], ARRAY[
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
                ]) WITH ORDINALITY AS staff (name, email, position)
ORDER BY staff.position;

INSERT INTO org (name)
VALUES ('Harbourview Dental'),
       ('Karoo Freight'),
       ('Mosaic Architects'),
       ('Fynbos Botanicals'),
       ('Northgate Accounting'),
       ('Summit Physiotherapy'),
       ('Blue Crane Logistics'),
       ('Oakridge Primary School'),
       ('Protea Property Group'),
       ('Riverside Veterinary Clinic'),
       ('Tafelberg Legal'),
       ('Ember Coffee Roasters'),
       ('Cedar & Stone Interiors'),
       ('Atlas Engineering'),
       ('Marula Lodge'),
       ('Greenpoint Motors'),
       ('Kestrel Insurance Brokers'),
       ('Lighthouse Media'),
       ('Silverleaf Pharmacy'),
       ('Baobab Consulting');
