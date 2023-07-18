CREATE TABLE lineitem (
    l_orderkey INT,
    l_partkey INT,
    l_suppkey INT,
    l_linenumber INT,
    l_quantity DECIMAL,
    l_extendedprice DECIMAL,
    l_discount DECIMAL,
    l_tax DECIMAL,
    l_returnflag CHAR(1),
    l_linestatus CHAR(1),
    l_shipdate DATE,
    l_commitdate DATE,
    l_receiptdate DATE,
    l_shipinstruct CHAR(25),
    l_shipmode CHAR(10),
    l_comment VARCHAR(44),
    PRIMARY KEY (l_orderkey, l_linenumber),
    FOREIGN KEY (l_orderkey) REFERENCES orders(o_orderkey),
    FOREIGN KEY (l_partkey) REFERENCES part(p_partkey),
    FOREIGN KEY (l_suppkey) REFERENCES supplier(s_suppkey),
    FOREIGN KEY (l_partkey, l_suppkey) REFERENCES partsupp(ps_partkey, ps_suppkey)
);

-- Import data into the lineitem table
.mode tabs
.separator '|'
.import 'data_100m/lineitem.tbl' lineitem


CREATE TABLE orders (
    o_orderkey INT PRIMARY KEY,
    o_custkey INT,
    o_orderstatus CHAR(1),
    o_totalprice DECIMAL,
    o_orderdate DATE,
    o_orderpriority CHAR(15),
    o_clerk CHAR(15),
    o_shippriority INT,
    o_comment VARCHAR(79),
    FOREIGN KEY (o_custkey) REFERENCES customer(c_custkey)
);


-- Import data into the orders table
.mode tabs
.separator '|'
.import 'data_100m/order.tbl' orders

CREATE TABLE customer (
    c_custkey INT PRIMARY KEY,
    c_name VARCHAR(25),
    c_address VARCHAR(40),
    c_nationkey INT,
    c_phone CHAR(15),
    c_acctbal DECIMAL,
    c_mktsegment CHAR(10),
    c_comment VARCHAR(117),
    FOREIGN KEY (c_nationkey) REFERENCES nation(n_nationkey)
);

-- Import data into the nation table
.mode tabs
.separator '|'
.import 'data_100m/customer.tbl' customer


CREATE TABLE nation (
    n_nationkey INT PRIMARY KEY,
    n_name CHAR(25),
    n_regionkey INT,
    n_comment VARCHAR(152),
    FOREIGN KEY (n_regionkey) REFERENCES region(r_regionkey)
);

-- Import data into the nation table
.mode tabs
.separator '|'
.import 'data_100m/nation.tbl' nation


CREATE TABLE region (
    r_regionkey INT PRIMARY KEY,
    r_name CHAR(25),
    r_comment VARCHAR(152)
);

-- Import data into the region table
.mode tabs
.separator '|'
.import 'data_100m/region.tbl' region

