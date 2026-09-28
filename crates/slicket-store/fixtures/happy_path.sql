WITH tenant_org AS (
    INSERT INTO org (name)
    VALUES ('Slinky IT')
    RETURNING org_id
)
INSERT INTO tenant (org_id)
SELECT org_id
FROM tenant_org;

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
