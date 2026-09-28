INSERT INTO org (name)
VALUES ('Slinky IT');

INSERT INTO tenant (org_id, name)
SELECT org_id, name
FROM org
WHERE name = 'Slinky IT';

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
