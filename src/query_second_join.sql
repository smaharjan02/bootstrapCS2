select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and L_LINENUMBER < 3 and L_LINENUMBER > 0
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and L_DISCOUNT < .07 and L_DISCOUNT > .02
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and L_EXTENDEDPRICE < 100000.00 and L_EXTENDEDPRICE > 20000.00
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and L_DISCOUNT < .04 and L_DISCOUNT> 0.0
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and L_QUANTITY < 20 and L_QUANTITY > 10
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and L_EXTENDEDPRICE < 15000.00 and L_EXTENDEDPRICE > 0.0
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and o_totalprice < 120000 and o_totalprice > 60000
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and o_totalprice < 180000 and o_totalprice > 100000
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and c_acctbal < 5000 and c_acctbal > 2000
select count(*) from lineitem,orders,customer where l_orderkey = o_orderkey and o_custkey = c_custkey and c_acctbal < 10000 and c_acctbal > 5000